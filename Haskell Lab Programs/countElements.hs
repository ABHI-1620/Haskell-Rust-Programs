countElements :: [a] -> Int
countElements [] = 0
countElements (_:xs) = 1 + countElements xs