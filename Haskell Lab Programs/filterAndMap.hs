filterAndMap :: (a -> Bool) -> (a -> b) -> [a] -> [b]
filterAndMap f g = map g . filter f

main :: IO ()
main = print (filterAndMap even (*2) [1, 2, 3, 4, 5, 6])