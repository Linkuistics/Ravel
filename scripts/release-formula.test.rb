# Run the production formula methods with Homebrew itself in a temporary prefix.
# API provenance: https://docs.brew.sh/rubydoc/Formula.html#test-instance_method
require "formula"
require "formula_assertions"
require "json"
require "tmpdir"

formula_path, source, prefix_path, test_path, versioned_prefix, versioned_test = ARGV
raise "expected FORMULA SOURCE PREFIX TESTPATH VERSIONED_PREFIX VERSIONED_TESTPATH" unless ARGV.length == 6
RECORD = "share/ravel/installation.json".freeze
# The archive holds no record; only the formula's installation writes one.
expected = (["bin", "share", RECORD] + Dir.glob("{bin,share}/**/*", base: source)).sort
raise "the archive holds an installation record" if File.exist?(Pathname(source)/RECORD)
assets = %w[examples/static.rhai examples/sample.rhai examples/llm.rhai notices/THIRD-PARTY-NOTICES.md]
expected_assets = assets.to_h { |asset| [asset, File.binread(Pathname(source)/"share/ravel"/asset)] }
expected_record = { "schemaVersion" => 1, "skillDirectory" => "#{HOMEBREW_PREFIX}/share/ravel/skills" }

# Install CLASS from FILE into PREFIX and run its test in TESTPATH. Return the
# record bytes. Construction: https://docs.brew.sh/rubydoc/Formula.html#initialize-instance_method
def install_and_test(klass, name, file, source, prefix, testpath, expected, expected_assets)
  prefix = Pathname(prefix)
  formula = klass.new(name, Pathname(file), :stable)
  # Same assertion extension as Homebrew's test runner; no substitute helpers.
  # https://github.com/Homebrew/brew/blob/7.0.9/Library/Homebrew/test.rb
  formula.extend(Homebrew::Assertions)
  formula.define_singleton_method(:prefix) { |*_args| prefix }
  # Pathname#install moves its files, so each installation takes a fresh copy.
  Dir.mktmpdir do |staged|
    FileUtils.cp_r("#{source}/.", staged, preserve: true)
    Dir.chdir(staged) { formula.install }
  end
  actual = Dir.glob("**/*", base: prefix).sort
  raise "#{name} installation differs from archive: #{actual.inspect}" unless actual == expected
  formula.instance_variable_set(:@testpath, Pathname(testpath))
  Dir.chdir(testpath) { formula.test }
  raise "#{name} did not test the front through a symlink" unless File.symlink?(Pathname(testpath)/"ravel")
  raise "#{name} did not test a private installation" unless File.file?(Pathname(testpath)/"relocated/bin/ravel")
  # Installation must preserve the notice and examples exactly, with one executable.
  expected_assets.each do |asset, bytes|
    raise "#{name} changed installed #{asset}" unless bytes == File.binread(prefix/"share/ravel"/asset)
  end
  raise "#{name} installed the retired libexec" if (prefix/"libexec").exist?
  executables = Dir.glob("**/*", base: prefix).select { |p| File.file?(prefix/p) && File.executable?(prefix/p) }
  raise "#{name} installed unexpected executables: #{executables.inspect}" unless executables == ["bin/ravel"]
  File.binread(prefix/RECORD)
end

# Load this owned file directly; Formulary.factory only accepts installed taps.
load formula_path
record = install_and_test(Ravel, "ravel", formula_path, source, prefix_path, test_path, expected, expected_assets)
raise "unexpected installation record #{record.inspect}" unless JSON.parse(record) == expected_record
puts "release-formula: actual Homebrew install/test methods passed from a private prefix"

# The keg-only major channel, as docs/specs/release-channels.md names it.
# This fixture derives it from the same rendered text until the channel
# tooling renders ravel@2.rb itself. Versions: https://docs.brew.sh/Versions
text = File.read(formula_path)
versioned = text.sub(/^class Ravel < Formula$/, "class RavelAT2 < Formula")
                .sub(/^(  license .*)$/, "\\1\n  keg_only :versioned_formula")
raise "cannot derive the versioned class" if versioned == text || !versioned.include?("keg_only")
versioned_path = Pathname(versioned_prefix).dirname/"ravel@2.rb"
versioned_path.write(versioned)
load versioned_path.to_s
raise "the versioned class is not keg-only" unless RavelAT2.new("ravel@2", versioned_path, :stable).keg_only?
versioned_record = install_and_test(RavelAT2, "ravel@2", versioned_path, source, versioned_prefix, versioned_test,
                                    expected, expected_assets)
raise "ravel@2 wrote another record: #{versioned_record.inspect}" unless versioned_record == record
puts "release-formula: the keg-only ravel@2 wrote the same installation record"
