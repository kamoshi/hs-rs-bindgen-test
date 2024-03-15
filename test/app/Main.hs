module Main where

import Foreign.C.String (withCString, peekCString)
import Greetings (hello_rust)

main :: IO ()
main = do
  res <- withCString "Haskell" hello_rust
  str <- peekCString res
  print str
