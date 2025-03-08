sumFilteredSquares :: [Int] -> Int
sumFilteredSquares = sum . map (^2) . filter (<=10)

main :: IO ()
main = print (sumFilteredSquares [5, 12, 9, 20, 15])