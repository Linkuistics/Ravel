# Run the production formula methods with Homebrew itself in a temporary prefix.
# API provenance: https://docs.brew.sh/rubydoc/Formula.html#test-instance_method
require "formula"
require "formula_assertions"

formula_path, source, prefix_path, test_path, probes_path = ARGV
raise "expected FORMULA SOURCE PREFIX TESTPATH PROBES" unless ARGV.length == 5
prefix_path = Pathname(prefix_path)
# Load this owned file directly; Formulary.factory only accepts installed taps.
# Formula construction: https://docs.brew.sh/rubydoc/Formula.html#initialize-instance_method
load formula_path
formula = Ravel.new("ravel", Pathname(formula_path), :stable)
# Same assertion extension as Homebrew's test runner; no substitute helpers.
# https://github.com/Homebrew/brew/blob/7.0.9/Library/Homebrew/test.rb
formula.extend(Homebrew::Assertions)
formula.define_singleton_method(:prefix) { |*_args| prefix_path }
expected = (["bin", "libexec"] + Dir.glob("{bin,libexec}/**/*", base: source)).sort
Dir.chdir(source) { formula.install }
actual = Dir.glob("**/*", base: prefix_path).sort
raise "formula installation differs from archive: #{actual.inspect}" unless actual == expected
formula.instance_variable_set(:@testpath, Pathname(test_path))
Dir.chdir(test_path) { formula.test }
raise "formula did not test the front through a symlink" unless File.symlink?(Pathname(test_path)/"ravel")
puts "release-formula: actual Homebrew install/test methods passed from a private prefix"

# A probe in the manifest's worker path has a valid layout but must refuse
# before running policy code. Exercise the archive-derived installed front.
worker = prefix_path/"libexec/ravel/ravel-policy"
saved_worker = Pathname(test_path)/"shipped-worker"
FileUtils.cp(worker, saved_worker)
%w[autoload tsconfig unregistered unmoved].each do |probe|
  FileUtils.cp(Pathname(probes_path)/probe/"ravel-policy", worker)
  command = ["/usr/bin/env", "-i", "PATH=/usr/bin:/bin",
             "HOME=#{test_path}/home", "TMPDIR=#{test_path}/tmp",
             (Pathname(test_path)/"ravel").to_s, "start-session", "--kind", "check",
             "--config", (Pathname(test_path)/"policy.ts").to_s, "--prompt", "probe", "--json"]
  report = JSON.parse(formula.shell_output("#{command.shelljoin} 2>&1", 5))
  formula.assert_equal "worker_identity_mismatch", report.fetch("error").fetch("code")
  raise "wrong probe refusal" unless report.fetch("error").fetch("message").include?("probe-#{probe}-")
  puts "release-formula: installed #{probe} probe refused with worker_identity_mismatch"
end
FileUtils.cp(saved_worker, worker)
