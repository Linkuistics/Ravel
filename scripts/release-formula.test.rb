# Run the production formula methods with Homebrew itself in a temporary prefix.
# API provenance: https://docs.brew.sh/rubydoc/Formula.html#test-instance_method
require "formula"
require "formula_assertions"

formula_path, source, prefix_path, test_path = ARGV
raise "expected FORMULA SOURCE PREFIX TESTPATH" unless ARGV.length == 4
prefix_path = Pathname(prefix_path)
# Load this owned file directly; Formulary.factory only accepts installed taps.
# Formula construction: https://docs.brew.sh/rubydoc/Formula.html#initialize-instance_method
load formula_path
formula = Ravel.new("ravel", Pathname(formula_path), :stable)
# Same assertion extension as Homebrew's test runner; no substitute helpers.
# https://github.com/Homebrew/brew/blob/7.0.9/Library/Homebrew/test.rb
formula.extend(Homebrew::Assertions)
formula.define_singleton_method(:prefix) { |*_args| prefix_path }
expected = (["bin", "share"] + Dir.glob("{bin,share}/**/*", base: source)).sort
assets = %w[examples/static.rhai examples/sample.rhai examples/llm.rhai notices/THIRD-PARTY-NOTICES.md]
expected_assets = assets.to_h { |asset| [asset, File.binread(Pathname(source)/"share/ravel"/asset)] }
Dir.chdir(source) { formula.install }
actual = Dir.glob("**/*", base: prefix_path).sort
raise "formula installation differs from archive: #{actual.inspect}" unless actual == expected
formula.instance_variable_set(:@testpath, Pathname(test_path))
Dir.chdir(test_path) { formula.test }
raise "formula did not test the front through a symlink" unless File.symlink?(Pathname(test_path)/"ravel")
puts "release-formula: actual Homebrew install/test methods passed from a private prefix"

# Installation must preserve the notice and examples exactly, with one executable.
expected_assets.each do |asset, bytes|
  raise "changed installed #{asset}" unless bytes == File.binread(prefix_path/"share/ravel"/asset)
end
raise "retired libexec installed" if (prefix_path/"libexec").exist?
executables = Dir.glob("**/*", base: prefix_path).select { |p| File.file?(prefix_path/p) && File.executable?(prefix_path/p) }
raise "unexpected executables: #{executables.inspect}" unless executables == ["bin/ravel"]
