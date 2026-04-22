class Cider < Formula
  desc "CLI tool for signing and sideloading iOS apps"
  homepage "https://github.com/Muonagi/cider-cli"
  version "0.1.6"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/Muonagi/cider-cli/releases/download/v0.1.6/cider-aarch64-apple-darwin.tar.gz"
      sha256 "4d2b127fed60e891dcf1a554af05ff175f98f46a4be0dbeefda87f85183d3a9a"
    else
      url "https://github.com/Muonagi/cider-cli/releases/download/v0.1.6/cider-x86_64-apple-darwin.tar.gz"
      sha256 "da470d9fbdf7b44e1855a9d638ac76703029957e21031e7fc9feadfed03eb98f"
    end
  end

  on_linux do
    url "https://github.com/Muonagi/cider-cli/releases/download/v0.1.6/cider-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "66e3fdef2d70933ac92cd6383f374430c9610a3b3e2aa9889c9f015e8d9dd9ce"
  end

  def install
    bin.install "cider"
  end

  test do
    assert_match "cider", shell_output("#{bin}/cider --help")
  end
end
