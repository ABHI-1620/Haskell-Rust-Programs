productOfSquares :: [Int] -> Int
productOfSquares = foldl (*) 1 . map (^2) . filter odd

main :: IO ()
main = print (productOfSquares [1, 2, 3, 4, 5, 6])