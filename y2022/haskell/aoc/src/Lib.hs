module Lib
    ( getMaxCal
    ) where

import Data.List
import Data.List.Split

toInt :: String -> Integer
toInt [] = 0
toInt s = read s :: Integer

getCalories :: [String] -> [[Integer]]
getCalories s = map (map toInt) $ splitWhen (==[]) s

getMaxCal :: FilePath -> IO (Integer, Integer)
getMaxCal path = do
  contents <- readFile path
  let cals = getCalories $ lines $ contents
  let ans1 = maximum (map sum cals)
  let ans2 = sum $ take 3 $ reverse $ sort (map sum cals)
  return (ans1,ans2)
