applyOp :: (Int -> Int -> Int) -> [Int] -> Int
applyOp _ [] = 0
applyOp op (x:xs) = foldl op x xs

sumOfSquaresEven :: [Int] -> Int
sumOfSquaresEven = applyOp (+) . map (^2) . filter even

main :: IO ()
main = print (sumOfSquaresEven [1, 2, 3, 4, 5, 6])