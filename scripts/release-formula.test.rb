# Run the production formula methods with Homebrew itself in a temporary prefix.
# API provenance: https://docs.brew.sh/rubydoc/Formula.html#test-instance_method
require "formula"
require "formula_assertions"
require "json"
require "tmpdir"

source, deprecated, *rest = ARGV
raise "expected SOURCE DEPRECATED then CHANNEL FORMULA PREFIX TESTPATH for each channel" unless !rest.empty? && (rest.length % 4).zero?
groups = rest.each_slice(4).to_h { |channel, file, prefix, testpath| [channel, [file, prefix, testpath]] }
raise "expected the channels versioned-2 and unversioned, got #{groups.keys.inspect}" unless groups.keys.sort == %w[unversioned versioned-2]
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

# docs/specs/release-channels.md: each channel's name, class and keg-only state.
# Versioned formulae: https://docs.brew.sh/Versions
CHANNELS = {
  "unversioned" => { name: "ravel", class_name: "Ravel", keg_only: false },
  "versioned-2" => { name: "ravel@2", class_name: "RavelAT2", keg_only: true },
}.freeze

# One release, two channels: the rendered texts differ only in the class line
# and the keg_only line.
texts = groups.transform_values { |file, _prefix, _testpath| File.read(file) }
normalise = lambda do |text|
  text.sub(/^class \w+ < Formula$/, "class CHANNEL < Formula").sub(/^  keg_only :versioned_formula\n/, "")
end
raise "the channel formulae differ beyond class and keg_only" unless normalise.(texts["unversioned"]) == normalise.(texts["versioned-2"])
raise "the unversioned formula is keg-only" if texts["unversioned"].match?(/^\s*keg_only\b/)
raise "the versioned formula is not keg-only" unless texts["versioned-2"].include?("\n  keg_only :versioned_formula\n")

records = groups.to_h do |channel, (file, prefix, testpath)|
  identity = CHANNELS.fetch(channel)
  raise "#{file} is not named #{identity[:name]}.rb" unless File.basename(file) == "#{identity[:name]}.rb"
  raise "#{file} does not define class #{identity[:class_name]}" unless texts[channel].match?(/^class #{identity[:class_name]} < Formula$/)
  # Load this owned file directly; Formulary.factory only accepts installed taps.
  load file
  klass = Object.const_get(identity[:class_name])
  raise "#{identity[:name]} keg_only? is not #{identity[:keg_only]}" unless klass.new(identity[:name], Pathname(file), :stable).keg_only? == identity[:keg_only]
  record = install_and_test(klass, identity[:name], file, source, prefix, testpath, expected, expected_assets)
  raise "#{identity[:name]} wrote an unexpected installation record #{record.inspect}" unless JSON.parse(record) == expected_record
  puts "release-formula: actual Homebrew install/test methods passed for #{identity[:name]} from a private prefix"
  [channel, record]
end
raise "the channels wrote different installation records" unless records.values.uniq.length == 1
puts "release-formula: both channels wrote the same installation record"

# Promotion: ravel@2.rb gains one deprecate! line after keg_only and changes
# nothing else. Homebrew's own loader must then report it deprecated in
# favour of ravel. A separate namespace keeps RavelAT2 above intact.
# https://github.com/Homebrew/brew/blob/7.0.13/Library/Homebrew/formulary.rb (from_contents)
deprecated_text = File.read(deprecated)
deprecation = deprecated_text.lines.grep(/^\s*deprecate!/)
raise "the deprecated formula has #{deprecation.length} deprecate! lines" unless deprecation.length == 1
unless deprecation[0].match?(/\A  deprecate! date: "\d{4}-\d{2}-\d{2}", because: :versioned_formula, replacement_formula: "ravel"\n\z/)
  raise "unexpected deprecation #{deprecation[0].inspect}"
end
raise "the deprecated formula changes more than its deprecate! line" unless deprecated_text.sub(deprecation[0], "") == texts["versioned-2"]
raise "deprecate! does not follow keg_only" unless deprecated_text.include?("\n  keg_only :versioned_formula\n#{deprecation[0]}")
promoted = Formulary.from_contents("ravel@2", Pathname(deprecated), deprecated_text)
raise "the promoted ravel@2 is not deprecated" unless promoted.deprecated?
raise "the promoted ravel@2 has reason #{promoted.deprecation_reason.inspect}" unless promoted.deprecation_reason == :versioned_formula
raise "the promoted ravel@2 names #{promoted.deprecation_replacement_formula.inspect}" unless promoted.deprecation_replacement_formula == "ravel"
raise "the promoted ravel@2 is not keg-only" unless promoted.keg_only?
puts "release-formula: Homebrew loads the promoted ravel@2 as deprecated in favour of ravel"
