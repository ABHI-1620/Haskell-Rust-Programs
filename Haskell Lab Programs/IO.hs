applyOp :: (Int -> Int -> Int) -> [Int] -> Int
applyOp _ [] = 0
applyOp op (x:xs) = foldl op x xs

main :: IO ()
main = do
    putStrLn "Enter operation (+ or *):"
    opStr <- getLine
    putStrLn "Enter first number:"
    num1 <- readLn
    putStrLn "Enter second number:"
    num2 <- readLn
    let op = if opStr == "+" then (+) else (*)
    print (applyOp op [num1, num2])