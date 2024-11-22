let upstream =
      https://github.com/purescript/package-sets/releases/download/psc-0.15.15/packages.dhall
        sha256:00f05148b768f69c5d5d9657051234b51d419b603e2a2de9ecd3eab5e63e86ab

let additions =
      { purs-utils = ../../libs/purs-utils/spago.dhall as Location }

in  upstream // additions
