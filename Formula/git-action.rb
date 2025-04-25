class GitAction < Formula
    desc "Git-Action"
    homepage "https://github.com/uname-n/git-action"
    url "https://github.com/uname-n/git-action/archive/refs/tags/0.0.1.tar.gz"
    sha256 "PLACEHOLDER_SHA256"
    license "MIT"
  
    depends_on "rust" => :build
  
    def install
      system "cargo", "install", *std_cargo_args
    end
  
    test do
      system "git-action", "--version"
    end
end
  