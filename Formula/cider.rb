class Cider < Formula
  desc "CLI tool for signing and sideloading iOS apps"
  homepage "https://github.com/Muonagi/cider-cli"
  version "0.1.1"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/Muonagi/cider-cli/releases/download/v#{version}/cider-aarch64-apple-darwin.tar.gz"
      sha256 "SHA256_MACOS_ARM64"
    else
      url "https://github.com/Muonagi/cider-cli/releases/download/v#{version}/cider-x86_64-apple-darwin.tar.gz"
      sha256 "SHA256_MACOS_X86_64"
    end
  end

  on_linux do
    url "https://github.com/Muonagi/cider-cli/releases/download/v#{version}/cider-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "SHA256_LINUX_X86_64"
  end

  def install
    bin.install "cider"
  end

  test do
    assert_match "cider", shell_output("#{bin}/cider --help")
  end
end
