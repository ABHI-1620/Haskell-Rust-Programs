module Main where

-- Function to calculate tax based on slabs
calculateTax :: [(Double, Double, Double)] -> Double -> Double
calculateTax slabs income = sum [min (income - lower) (upper - lower) * rate | (lower, upper, rate) <- slabs, income > lower]

-- Old Tax Regime slabs
oldTaxSlabs :: [(Double, Double, Double)]
oldTaxSlabs = [(250000, 500000, 0.05), (500000, 1000000, 0.20), (1000000, 1e10, 0.30)] -- Using a large number instead of infinity

-- New Tax Regime slabs
newTaxSlabs :: [(Double, Double, Double)]
newTaxSlabs = [(250000, 500000, 0.05), (500000, 750000, 0.10), (750000, 1000000, 0.15), (1000000, 1250000, 0.20), (1250000, 1500000, 0.25), (1500000, 1e10, 0.30)]

-- Main function
main :: IO ()
main = do
    putStrLn "Enter your annual income:"
    income <- readLn :: IO Double

    if income <= 250000 then
        putStrLn "Your calculated tax is: 0.0 (Income below the taxable limit)"
    else do
        putStrLn "Choose the tax regime:"
        putStrLn "1. Old Tax Regime"
        putStrLn "2. New Tax Regime"
        choice <- getLine
        let slabs = case choice of
                      "1" -> oldTaxSlabs
                      "2" -> newTaxSlabs
                      _   -> error "Invalid choice. Please choose 1 or 2."
        let tax = calculateTax slabs income
        putStrLn $ "Your calculated tax is: " ++ show tax
