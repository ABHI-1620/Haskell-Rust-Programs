doubleFilteredProduct :: [Int] -> Int
doubleFilteredProduct = foldl (*) 1 . map (*2) . filter (<=10)

main :: IO ()
main = print (doubleFilteredProduct [5, 12, 9, 20, 15])