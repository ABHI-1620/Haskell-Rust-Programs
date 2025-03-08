square :: Int -> Int
square x = x * x

transformList :: [Int] -> [Int]
transformList xs = map (+10) (map square xs)
