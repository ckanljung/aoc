module Lib
    ( dec01
    ) where

import Data.List
import Data.List.Split

toInt :: String -> Integer
toInt [] = 0
toInt s = read s :: Integer

data Hand =
    Rock |
    Paper |
    Scissors

instance Show Hand where  
    show Rock = "Rock"  
    show Paper = "Paper"  
    show Scissors = "Scissors"  

data Result =
    Win |
    Lose |
    Draw

instance Show Result where  
    show Lose = "lose"  
    show Win = "Win"  
    show Draw = "Draw"  

getCalories :: [String] -> [[Integer]]
getCalories s = map (map toInt) $ splitWhen (==[]) s

parseHand :: String -> Hand
parseHand "A" = Rock
parseHand "B" = Paper
parseHand "C" = Scissors
parseHand _ = Rock

parseResult::String -> Result
parseResult "X" = Lose
parseResult "Y" = Draw
parseResult "Z" = Win
parseResult _ = Lose

parseStrategy :: [String] -> (Hand,Hand)
parseStrategy [] = (Rock,Rock)
parseStrategy (e:[]) = (parseHand(e),Rock)
parseStrategy (e:f:_) = (parseHand(e),parseHand(f))

parseStrategy2 :: [String] -> (Hand,Result)
parseStrategy2 [] = (Rock,Lose)
parseStrategy2 (e:[]) = (parseHand(e),Lose)
parseStrategy2 (e:f:_) = (parseHand(e),parseResult(f))

evalStrategy :: (Hand,Hand) -> Integer
evalStrategy (Rock,Scissors) = 0 + 3
evalStrategy (Rock,Rock) = 3 + 1
evalStrategy (Rock,Paper) = 6 + 2
evalStrategy (Paper,Rock) = 0 + 1
evalStrategy (Paper,Paper) = 3 + 2
evalStrategy (Paper,Scissors) = 6 + 3
evalStrategy (Scissors,Paper) = 0 + 2
evalStrategy (Scissors,Scissors) = 3 + 3
evalStrategy (Scissors,Rock) = 6 + 1

evalStrategy2 :: (Hand,Result) -> Hand
evalStrategy2 (Rock,Lose) = Scissors
evalStrategy2 (Rock,Win) = Paper
evalStrategy2 (Rock,Draw) = Rock
evalStrategy2 (Paper,Lose) = Rock
evalStrategy2 (Paper,Win) = Scissors
evalStrategy2 (Paper,Draw) = Paper
evalStrategy2 (Scissors,Lose) = Paper
evalStrategy2 (Scissors,Win) = Rock
evalStrategy2 (Scissors,Draw) = Scissors

dec01 :: FilePath -> IO (Integer, Integer)
dec01 path = do
  contents <- readFile path
  let cals = getCalories $ lines $ contents
  let ans1 = maximum (map sum cals)
  let ans2 = sum $ take 3 $ reverse $ sort (map sum cals)
  return (ans1,ans2)

dec02 :: FilePath -> IO (Integer, Integer)
dec02 path = do
  contents <- readFile path
  let strategy = map words $ lines $ contents
  let ans1 = sum (map evalStrategy $ map parseStrategy strategy)
  let strats = map parseStrategy2 strategy
  let elfHand = map (\(x, y) -> x) strats
  let ans2 = sum $ map evalStrategy $ zip elfHand (map evalStrategy2 strats)
  return (ans1,ans2)

testlist = [["A","Y"],["B","X"],["C","Z"]]
