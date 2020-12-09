module Dec06 where

import Data.List
import Data.List.Split
import qualified Data.Set as Set

nubOrd :: Ord a => [a] -> [a] 
nubOrd xs = go Set.empty xs where
  go s (x:xs)
   | x `Set.member` s = go s xs
   | otherwise        = x : go (Set.insert x s) xs
  go _ _              = []

intoWords :: [[Char]] -> [[Char]]
intoWords x = map (\x -> (intercalate "" x)) $ splitWhen null x

uniqCnt :: [[Char]] -> [Int]
uniqCnt x = map length $ map nubOrd $ map (\x -> (intercalate "" x)) $ splitWhen null x

getWords :: FilePath -> IO [[Char]]
getWords path = do
  contents <- readFile path
  let words = map nubOrd $ intoWords $ lines $ contents
  return words

antersect l = foldr (\a b ->  Set.elems $ Set.fromList a `Set.intersection` Set.fromList b) (head l) l

getsInter :: FilePath -> IO ([[Char]], Int)
getsInter path = do
  contents <- readFile path
  let inters = map antersect $ splitWhen null $ lines $ contents
  let csum = sum $ map length $ inters
  return (inters, csum)

getsUniqSum :: FilePath -> IO ([Int], Int)
getsUniqSum path = do
  contents <- readFile path
  let uniqs = uniqCnt $ lines $ contents
  let csum = sum uniqs
  return (uniqs, csum)

testin = ["abc","","a","b","c","","ab","ac","","a","a","a","a","","b"]

