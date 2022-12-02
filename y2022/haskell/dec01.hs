module Dec01 where

getLines :: FilePath -> IO [String]
getLines path = do
  contents <- readFile path
  let lns = lines $ contents
  return lns
