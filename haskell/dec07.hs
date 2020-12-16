module Dec07 where

import Data.List
import Data.List.Split
import qualified Data.Set as Set

import qualified Data.Text as T
import Data.Text.Encoding as E
import Data.ByteString (ByteString)

data Bag = 
  ColoredBag String String |
  SimpleBag String |
  EmptyBag deriving (Show)  

wordsToBag :: [[Char]] -> Bag
wordsToBag [] = EmptyBag
wordsToBag [a] = SimpleBag a
wordsToBag (a : b : _) = ColoredBag a b

parseContent :: [T.Text] -> [T.Text]
parseContent a = T.splitOn (T.pack ", ") $ T.strip $ head a

splitLine :: [Char] -> (Bag,[T.Text])
splitLine x = (wordsToBag $ words $ T.unpack $ head a, parseContent $ tail a) 
  where a = T.splitOn (T.pack "contain") (T.pack x)

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

getsUniqSum :: FilePath -> IO ([Int], Int)
getsUniqSum path = do
  contents <- readFile path
  let uniqs = uniqCnt $ lines $ contents
  let csum = sum uniqs
  return (uniqs, csum)

testin = ["abc","","a","b","c","","ab","ac","","a","a","a","a","","b"]


testa = "light red bags contain 1 bright white bag, 2 muted yellow bags."
testb="faded blue bags contain no other bags."
testc="bright white bags contain 1 shiny gold bag."
