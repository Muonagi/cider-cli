class Cider < Formula
  desc "CLI tool for signing and sideloading iOS apps"
  homepage "https://github.com/Muonagi/cider-cli"
  version "0.1.5"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/Muonagi/cider-cli/releases/download/v0.1.5/cider-aarch64-apple-darwin.tar.gz"
      sha256 "4ee5686b4183cdd9c76f90da32e451f7ac332e19daea18c3426b9425b282b9aa"
    else
      url "https://github.com/Muonagi/cider-cli/releases/download/v0.1.5/cider-x86_64-apple-darwin.tar.gz"
      sha256 "9530c9e6df8ceb10dfe0cda2d00dac839f665c23ef15d1866cae2868b9a29455"
    end
  end

  on_linux do
    url "https://github.com/Muonagi/cider-cli/releases/download/v0.1.5/cider-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "8feea9ca4696daa27d501406beb9b36a0c7e99e63f43c2d50d0ce27481861ac3"
  end

  def install
    bin.install "cider"
  end

  test do
    assert_match "cider", shell_output("#{bin}/cider --help")
  end
end
