_gh="Lycoris-69-rs"

pkgname=yt-dlp-wrapper-tui
pkgver=1.1.0
pkgrel=1
pkgdesc='TUI wrapper for yt-dlp with download queue and progress'
arch=('x86_64' 'aarch64')
url="https://github.com/$_gh/$pkgname"
license=('MIT')
depends=(
  'gcc-libs'
  'glibc'
  'yt-dlp'
  'ffmpeg'
)
makedepends=('cargo')
provides=("$pkgname")
conflicts=("$pkgname-git")
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

prepare() {
  cd "$pkgname-$pkgver"

  cargo fetch --locked
}

build() {
  cd "$pkgname-$pkgver"

  cargo build --release --locked
}

package() {
  cd "$pkgname-$pkgver"

  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"

  if [[ -f LICENSE ]]; then
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  fi
}
