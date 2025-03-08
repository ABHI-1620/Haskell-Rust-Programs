filterAndFold :: (a -> Bool) -> (a -> a -> a) -> [a] -> a
filterAndFold _ _ [] = error "Empty list"
filterAndFold f op (x:xs) = foldl op x (filter f xs)

main :: IO ()
main = print (filterAndFold odd (+) [1, 2, 3, 4, 5, 6])
