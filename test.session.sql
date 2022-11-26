-- @block
CREATE TABLE Transactions(
    AccountName VARCHAR(255),
    AvailableBalance INT,
    Amount INT,
    CategoryId INT,
    CleanedTransactionText TEXT,
    PaymentMethod VARCHAR(255),
    TransactionDate TIMESTAMP,
    CaseId INT PRIMARY KEY AUTO_INCREMENT,
    AccountTypeId INT
);
-- @block 
DROP TABLE Transactions;

-- @block
-- Inserts a valid transaction 
INSERT INTO Transactions (
    AccountName, Amount, CategoryId, TransactionDate
    ) 
VALUES ('Fandango', 141, 72, '2022-10-21 00:00:01')

-- @block
/* Selects relevant information from 
categorized investments in the last 3 months */
SELECT AccountName, AvailableBalance, Amount, 
CleanedTransactionText, PaymentMethod, CaseId, AccountTypeId
FROM Transactions
WHERE CategoryId = 72 
AND TransactionDate BETWEEN '2022-08-21 00:00:00' AND '2022-11-20 23:59:59'
LIMIT 10000;
-- @block
-- Deletes the row with CaseId = 1
DELETE FROM Transactions WHERE CaseId = 1;
-- @block
-- Removes all entries without dropping table
DELETE FROM Transactions;

-- @block
/* Selects CaseId for all uncategorized transactions 
containing the text 'ok a.m.b.a' */
SELECT CaseId FROM Transactions 
WHERE CategoryId = 0 
AND CleanedTransactionText LIKE '%ok a.m.b.a%'
LIMIT 200;

-- @block
INSERT INTO Transactions (CategoryId, CleanedTransactionText)
VALUES (0, 'Montgomery Gas Station ok a.m.b.a payment successful')

-- @block
SELECT CaseId, CleanedTransactionText FROM Transactions 
WHERE Amount >= 0 
AND (
    (CleanedTransactionText LIKE '%budget%' AND CategoryId != 480) or 
    (CleanedTransactionText NOT LIKE '%budget%' AND CategoryId = 480)
    )
LIMIT 10000;


-- @block
INSERT INTO Transactions (Amount, CleanedTransactionText, CategoryId)
VALUES (1, "this is a bUDGET transfer", 72);