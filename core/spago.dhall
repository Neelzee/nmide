{-
Welcome to a Spago project!
You can edit this file as you like.

Need help? See the following resources:
- Spago documentation: https://github.com/purescript/spago
- Dhall language tour: https://docs.dhall-lang.org/tutorials/Language-Tour.html

When creating a new Spago project, you can use
`spago init --no-comments` or `spago init -C`
to generate this file without the comments in this block.
-}
{ name = "core"
, dependencies =
  [ "argonaut"
  , "argonaut-codecs"
  , "argonaut-generic"
  , "arrays"
  , "console"
  , "control"
  , "effect"
  , "either"
  , "foreign-object"
  , "lists"
  , "maybe"
  , "prelude"
  , "strings"
  , "transformers"
  , "tuples"
  , "web-dom"
  , "web-events"
  , "web-html"
  ]
, packages = ./packages.dhall
, sources = [ "app/**/*.purs" ]
}
