class Hmon < Formula
  desc "Fast, interactive, terminal-based system resource monitor written in Rust"
  homepage "https://github.com/hkatagal/hmon"
  url "https://static.crates.io/crates/hmon/hmon-0.1.0.crate"
  sha256 "ac6c4ec01a4d1e6b99dee2ef252b536d9fa66f807c686d3e1bd9355594396258"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "hmon", shell_output("#{bin}/hmon --help", 2)
  end
end
