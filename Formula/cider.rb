class Cider < Formula
  desc "CLI tool for signing and sideloading iOS apps"
  homepage "https://github.com/Muonagi/cider-cli"
  version "0.1.4"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/Muonagi/cider-cli/releases/download/v0.1.3/cider-aarch64-apple-darwin.tar.gz"
      sha256 "4ca165429388cea3bcf6787fcefbea54d5ddafef760ea6260c36908a0d944531"
    else
      url "https://github.com/Muonagi/cider-cli/releases/download/v0.1.3/cider-x86_64-apple-darwin.tar.gz"
      sha256 "8d1c8bddbb8d8173f07860eacdfe9369b28c8ffebdfacfe4860378c10ab28e0a"
    end
  end

  on_linux do
    url "https://github.com/Muonagi/cider-cli/releases/download/v0.1.3/cider-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "3435ebbec4cc9065ebf5e82e9ed1838491ab3cb06023319bfa3dd1ee50f649a4"
  end

  def install
    bin.install "cider"
  end

  test do
    assert_match "cider", shell_output("#{bin}/cider --help")
  end
end
