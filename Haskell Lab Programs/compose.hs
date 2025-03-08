compose :: (b -> c) -> (a -> b) -> (a -> c)
compose f g = \x -> f (g x)

transformation :: Int -> Int
transformation = compose (*2) (subtract 3)

main :: IO ()
main = print (map transformation [1, 2, 3, 4])