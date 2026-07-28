class Hmon < Formula
  desc "Fast, interactive, terminal-based system resource monitor written in Rust"
  homepage "https://github.com/hkatagal/hmon"
  url "https://github.com/hkatagal/hmon/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "PENDING_RELEASE_SHA256"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "hmon", shell_output("#{bin}/hmon --help", 2)
  end
end
