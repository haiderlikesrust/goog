# Maintainer: Haider <haideremailsender122@gmail.com>
pkgname=goog
pkgver=0.2.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="Search on google from your terminal"
license=('MIT OR Apache-2.0')

build() {
    return 0
}

package() {
    cargo install --root="$pkgdir" goog
}
