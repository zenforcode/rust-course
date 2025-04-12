# Writing Code That Lasts: A Practical Look at SOLID Principles

When we talk about writing maintainable, flexible, and scalable software, one set of principles frequently comes up: **SOLID**.

**SOLID** is an acronym that represents five core principles of object-oriented design. Introduced by Robert C. Martin—commonly known as *Uncle Bob*—these practices have been around for nearly 15 years. Yet, many developers are still unfamiliar with them, or unsure how to apply them effectively.

In this post, we'll explore what SOLID is all about, why it matters, and how to approach it wisely.

---

## What is SOLID?

At its core, SOLID is a set of design guidelines aimed at making software more understandable, flexible, and easier to maintain. Each letter stands for a specific principle:

- **S — Single Responsibility Principle**  
  A class should have one, and only one, reason to change.

- **O — Open/Closed Principle**  
  Software entities should be open for extension, but closed for modification.

- **L — Liskov Substitution Principle**  
  Objects should be replaceable with instances of their subtypes without affecting the correctness of the program.

- **I — Interface Segregation Principle**  
  No client should be forced to depend on methods it does not use.

- **D — Dependency Injection (or Dependency Inversion Principle)**  
  High-level modules should not depend on low-level modules. Both should depend on abstractions.

Each of these principles offers real value on its own. Together, they encourage a structure that naturally supports change, reuse, and collaboration.

---

## The Real Art: When to Use Them

While SOLID provides powerful tools, it’s important to remember they are *just that*—tools. The craft of software development lies not only in knowing **how** these principles work, but also in understanding **when** and **where** to apply them.

Overuse of design patterns and principles can lead to code that’s overly complex, abstracted to a point where it’s difficult to follow or maintain. It may be technically “flexible,” but impractical in real-world scenarios.

One of the most critical aspects of good code is **readability**. In most professional settings, software is built by **teams**, not individuals. Writing code that others can easily understand and build upon is just as important as making it technically sound.

---

## A Balanced Approach

Mastering the SOLID principles means more than memorizing definitions—it’s about applying them thoughtfully. Use them to guide your decisions, but always with the goal of keeping your code clean, clear, and collaborative.

Remember: patterns and practices are there to help, not to impress. The best code isn’t the most “clever”—it’s the code that people can work with, maintain, and grow over time.

---

# The single responsibility principle

The Single Responsibility Principle (SRP) is a foundational concept in software design. It states that a class should have one and only one reason to change, meaning it should encapsulate a single responsibility. When a class assumes multiple responsibilities, it becomes more difficult to maintain, test, and extend. Each added responsibility increases the likelihood that a change to one part of the class will inadvertently affect another, potentially unrelated, functionality.

To adhere to SRP, developers should refactor classes with multiple concerns into smaller, focused classes—each dedicated to a specific responsibility. This process often involves the use of delegation and abstraction, where a complex class delegates portions of its behavior to other specialized classes.

Delegating to well-defined abstractions is critical for building maintainable and extensible systems.


## Problem statement

To demonstrate the implications of violating SRP, consider the following example: a batch processor class that reads trade records from a file and inserts them into a database. At first glance, the class may seem manageable, but as business requirements evolve and additional features are introduced, this tightly coupled design quickly becomes problematic.

The code listing below presents the initial implementation of the batch processor:

```rust
use std::io::{BufRead, BufReader, Read};
use std::collections::VecDeque;
use sqlx::{query, PgPool};
use anyhow::{Result, Context};

const LOT_SIZE: i32 = 1000;

#[derive(Debug)]
struct TradeRecord {
    source_currency: String,
    destination_currency: String,
    lots: i32,
    price: f64,
}

pub struct TradeProcessor;

impl TradeProcessor {
    pub async fn process_trades<R: Read>(reader: R, db_pool: &PgPool) -> Result<()> {
        let mut trades: Vec<TradeRecord> = Vec::new();
        let reader = BufReader::new(reader);
        
        for (line_num, line) in reader.lines().enumerate() {
            let line = line?;
            let fields: Vec<&str> = line.split(',').collect();

            if fields.len() != 3 {
                println!("WARN: Line {} malformed. Only {} field(s) found.", line_num + 1, fields.len());
                continue;
            }

            if fields[0].len() != 6 {
                println!("WARN: Trade currencies on line {} malformed: '{}'", line_num + 1, fields[0]);
                continue;
            }

            let trade_amount = match fields[1].trim().parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    println!("WARN: Trade amount on line {} not a valid integer: '{}'", line_num + 1, fields[1]);
                    continue;
                }
            };

            let trade_price = match fields[2].trim().parse::<f64>() {
                Ok(val) => val,
                Err(_) => {
                    println!("WARN: Trade price on line {} not a valid decimal: '{}'", line_num + 1, fields[2]);
                    continue;
                }
            };

            let source_currency = &fields[0][..3];
            let destination_currency = &fields[0][3..];

            trades.push(TradeRecord {
                source_currency: source_currency.to_string(),
                destination_currency: destination_currency.to_string(),
                lots: trade_amount / LOT_SIZE,
                price: trade_price,
            });
        }

        // Insert into database
        let mut tx = db_pool.begin().await?;

        for trade in &trades {
            query!(
                r#"CALL insert_trade($1, $2, $3, $4)"#,
                trade.source_currency,
                trade.destination_currency,
                trade.lots,
                trade.price
            )
            .execute(&mut *tx)
            .await
            .context("Failed to execute trade insertion")?;
        }

        tx.commit().await?;

        println!("INFO: {} trades processed", trades.len());

        Ok(())
    }
}

```

## SRP Violations in the `TradeProcessor` Class

Although the `TradeProcessor` class is relatively compact, it clearly violates the **Single Responsibility Principle (SRP)** by managing multiple concerns within a single method. These responsibilities include:

- **File parsing and validation**
- **Business logic transformation** (e.g., converting trade amounts to lots)
- **Database transaction management and data persistence**
- **Logging and error reporting**

Each of these aspects represents a distinct responsibility. As such, changes in any one of them would require modifications to the `TradeProcessor`, potentially affecting unrelated functionality and increasing the risk of regression.

In the sections that follow, we will examine how to refactor this class by isolating its responsibilities into distinct components, each with a well-defined purpose. This restructuring will improve maintainability, enhance clarity, and ensure the class adheres to SRP-compliant design principles.

---

### A Closer Look: Too Many Responsibilities in a Single Method

This example not only highlights a class that encompasses too many responsibilities but also serves as a textbook case of a **method** that is doing too much. By examining the code carefully, we can identify at least five distinct operations within the `process_trades` method:

1. **Reading Input**  
   It reads each line from a `Stream`-like parameter and stores the lines.

2. **Parsing and Structuring Data**  
   It parses individual fields from each line and stores them in structured `TradeRecord` instances.

3. **Validation and Logging**  
   It performs inline validation and logs warnings directly to the console.

4. **Business Logic**  
   It applies transformation logic (e.g., dividing the trade amount by a lot size constant).

5. **Data Persistence**  
   It iterates over the parsed data and invokes a stored procedure to insert each trade into a database.

Each of these operations represents a different reason for the class to change, violating the core principle of SRP.

---

### Why This Design Is Problematic

The `TradeProcessor` class may need to change under any of the following circumstances:

- 📦 **Input Source Changes**  
  You might need to read data from a web service instead of a file stream.

- 🔄 **Input Format Evolves**  
  The format may change—for example, a new field such as `broker` could be added.

- ✅ **Validation Rules Update**  
  New or more complex validation rules might be introduced.

- 📣 **Logging Behavior Changes**  
  Instead of logging to the console, logs might need to be routed to a logging service, file, or external monitoring tool—especially in a cloud-hosted environment.

- 🗄️ **Database Layer Modifications**  
  The database structure or persistence mechanism might change. For instance:
  - A new field is added to the stored procedure.
  - You switch from a relational database to a document store.
  - Trade data must now be posted to a web API instead of being inserted directly.

Each of these changes would force updates to the `TradeProcessor` class, creating a tightly coupled and brittle design.

---

> Imagine the complexity and potential maintenance overhead of adding a feature where trades must be stored in a remote web service **only if a specific command-line argument is provided**. In its current form, `TradeProcessor` cannot support such extensibility without significant changes.

---

By refactoring this class and distributing its responsibilities into purpose-driven components—such as `TradeReader`, `TradeParser`, `TradeValidator`, `TradeLogger`, and `TradeRepository`—we can achieve a cleaner, more modular architecture. This not only supports the **open/closed principle** but also positions the system to scale gracefully as new requirements emerge.


### Refactoring for clarity

The first task on the road to refactoring the TradeProcessor so that it has one reason to change is to split the ProcessTrades method into smaller pieces so that each one focuses on a single responsibility. Each of the following listings shows a single method from the refactored TradeProcessor class, followed by an explanation of the changes.

First, Listing 5-2 shows the ProcessTrades method, which now does nothing more than delegate to other methods.

LISTING 5-2 The ProcessTrades method is very minimal because it delegates work to other methods.

Click here to view code image

public void ProcessTrades(System.IO.Stream stream)
{
    var lines = ReadTradeData(stream);
    var trades = ParseTrades(lines);
    StoreTrades(trades);
}

The original code was characterized by three distinct parts of a process—reading the trade data from a stream, converting the string data in the stream to TradeRecord instances, and writing the trades to persistent storage. Note that the output from one method feeds into the input to the next method. You cannot call StoreTrades until you have the trade records returned from the Parse-Trades method, and you cannot call ParseTrades until you have the lines returned from the ReadTradeData method.

Taking each of these methods in order, let’s look at ReadTradeData, in Listing 5-3.

LISTING 5-3 ReadTradeData encapsulates the original code.

Click here to view code image

private IEnumerable<string> ReadTradeData(System.IO.Stream stream)
{
    var tradeData = new List<string>();
    using (var reader = new System.IO.StreamReader(stream))
    {
        string line;
        while ((line = reader.ReadLine()) != null)
        {
            tradeData.Add(line);
        }
    }
    return tradeData;
}

This code is preserved from the original implementation of the ProcessTrades method. It has simply been encapsulated in a method that returns the resultant string data as a string enumeration. Note that this makes the return value read-only, whereas the original implementation unnecessarily allowed subsequent parts of the process to add further lines.

The ParseTrades method, shown in Listing 5-4, is next. It has changed somewhat from the original implementation because it, too, delegates some tasks to other methods.

LISTING 5-4 ParseTrades delegates to other methods to limit its complexity.

Click here to view code image

private IEnumerable<TradeRecord> ParseTrades(IEnumerable<string> tradeData)
{
    var trades = new List<TradeRecord>();
    var lineCount = 1;
    foreach (var line in tradeData)
    {
        var fields = line.Split(new char[] { ',' });

        if(!ValidateTradeData(fields, lineCount))
        {
            continue;
        }

        var trade = MapTradeDataToTradeRecord(fields);

        trades.Add(trade);

        lineCount++;
    }
    return trades;
}

This method delegates validation and mapping responsibilities to other methods. Without this delegation, this section of the process would still be too complex and it would retain too many responsibilities. The ValidateTradeData method, shown in Listing 5-5, returns a Boolean value to indicate whether any of the fields for a trade line are invalid.

LISTING 5-5 All of the validation code is in a single method.

Click here to view code image

private bool ValidateTradeData(string[] fields, int currentLine)
{
    if (fields.Length != 3)
    {
        LogMessage("WARN: Line {0} malformed. Only {1} field(s) found.", currentLine,
  fields.Length);
        return false;
    }

    if (fields[0].Length != 6)
    {
        LogMessage("WARN: Trade currencies on line {0} malformed: '{1}'", currentLine,
  fields[0]);
        return false;
    }

    int tradeAmount;
    if (!int.TryParse(fields[1], out tradeAmount))
    {
        LogMessage("WARN: Trade amount on line {0} not a valid integer: '{1}'",
  currentLine, fields[1]);
        return false;
    }

    decimal tradePrice;
    if (!decimal.TryParse(fields[2], out tradePrice))
    {
        LogMessage("WARN: Trade price on line {0} not a valid decimal: '{1}'",
  currentLine, fields[2]);
        return false;
    }

    return true;
}

The only change made to the original validation code is that it now delegates to yet another method for logging messages. Rather than embedding calls to Console.WriteLine where needed, the LogMessage method is used, shown in Listing 5-6.

LISTING 5-6 The LogMessage method is currently just a synonym for Console.WriteLine.

Click here to view code image

private void LogMessage(string message, params object[] args)
{
    Console.WriteLine(message, args);
}

Returning up the stack to the ParseTrades method, Listing 5-7 shows the other method to which it delegates. This method maps an array of strings representing the individual fields from the stream to an instance of the TradeRecord class.

LISTING 5-7 Mapping from one type to another is a separate responsibility.

Click here to view code image

private TradeRecord MapTradeDataToTradeRecord(string[] fields)
{
    var sourceCurrencyCode = fields[0].Substring(0, 3);
    var destinationCurrencyCode = fields[0].Substring(3, 3);
    var tradeAmount = int.Parse(fields[1]);
    var tradePrice = decimal.Parse(fields[2]);

    var tradeRecord = new TradeRecord
    {
        SourceCurrency = sourceCurrencyCode,
        DestinationCurrency = destinationCurrencyCode,
        Lots = tradeAmount / LotSize,
        Price = tradePrice
    };

    return tradeRecord;
}

The sixth and final new method introduced by this refactor is StoreTrades, shown in Listing 5-8. This method wraps the code for interacting with the database. It also delegates the informational log message to the aforementioned LogMessage method.

LISTING 5-8 With the StoreTrades method in place, the responsibilities in this class are clearly demarcated.

Click here to view code image

private void StoreTrades(IEnumerable<TradeRecord> trades)
{
    using (var connection = new System.Data.SqlClient.SqlConnection("Data
  Source=(local);Initial Catalog=TradeDatabase;Integrated Security=True"))
    {
        connection.Open();
        using (var transaction = connection.BeginTransaction())
        {
            foreach (var trade in trades)
            {
                var command = connection.CreateCommand();
                command.Transaction = transaction;
                command.CommandType = System.Data.CommandType.StoredProcedure;
                command.CommandText = "dbo.insert_trade";
                command.Parameters.AddWithValue("@sourceCurrency", trade.SourceCurrency);
                command.Parameters.AddWithValue("@destinationCurrency",
  trade.DestinationCurrency);
                command.Parameters.AddWithValue("@lots", trade.Lots);
                command.Parameters.AddWithValue("@price", trade.Price);

                command.ExecuteNonQuery();
            }

            transaction.Commit();
        }
        connection.Close();
    }

    LogMessage("INFO: {0} trades processed", trades.Count());
}

Looking back at this refactor, it is a clear improvement on the original implementation. However, what have you really achieved? Although the new ProcessTrades method is indisputably smaller than the monolithic original, and the code is definitely more readable, you have gained very little by way of adaptability. You can change the implementation of the LogMessage method so that it, for example, writes to a file instead of to the console, but that involves a change to the TradeProcessor class, which is precisely what you wanted to avoid.

This refactor has been an important stepping stone on the path to truly separating the responsibilities of this class. It has been a refactor for clarity, not for adaptability. The next task is to split each responsibility into different classes and place them behind interfaces. What you need is true abstraction to achieve useful adaptability.
Refactoring for abstraction

Building on the new TradeProcessor implementation, the next refactor introduces several abstractions that will allow you to handle almost any change request for this class. Although this running example might seem very small, perhaps even insignificant, it is a workable contrivance for the purposes of this tutorial. Also, it is very common for a small application such as this to grow into something much larger. When a few people start to use it, the feature requests begin to increase.

Often, the terms prototype and proof of concept are applied to such allegedly small applications, and the conversion from prototype to production application is relatively seamless. This is why the ability to refactor toward abstraction is such a touchstone of adaptive development. Without it, the myriad requests devolve into a “big ball of mud”—a class, or a group of classes in an assembly, with little delineation of responsibility and no discernible abstractions. The result is an application that has no unit tests and that is difficult to maintain and enhance, and yet that could be a critical piece of the line of business.

The first step in refactoring the TradeProcessor for abstraction is to design the interface or interfaces that it will use to perform the three high-level tasks of reading, processing, and storing the trade data. Figure 5-1 shows the first set of abstractions.
Image

FIGURE 5-1 The TradeProcessor will now depend on three new interfaces.

Because you moved all of the code from ProcessTrades into separate methods in the first refactor, you should have a good idea of where the first abstractions should be applied. As prescribed by the single responsibility principle, the three main responsibilities will be handled by different classes. As you know from previous chapters, you should not have direct dependencies from one class to another but should instead work via interfaces. Therefore, the three responsibilities are factored out into three separate interfaces. Listing 5-9 shows how the TradeProcessor class looks after this change.

LISTING 5-9 The TradeProcessor is now the encapsulation of a process, and nothing more.

Click here to view code image

public class TradeProcessor
{
    public TradeProcessor(ITradeDataProvider tradeDataProvider, ITradeParser tradeParser,
  ITradeStorage tradeStorage)
    {
        this.tradeDataProvider = tradeDataProvider;
        this.tradeParser = tradeParser;
        this.tradeStorage = tradeStorage;
    }

    public void ProcessTrades()
    {
        var lines = tradeDataProvider.GetTradeData();
        var trades = tradeParser.Parse(lines);
        tradeStorage.Persist(trades);
    }

    private readonly ITradeDataProvider tradeDataProvider;
    private readonly ITradeParser tradeParser;
    private readonly ITradeStorage tradeStorage;
}

The class is now significantly different from its previous incarnation. It no longer contains the implementation details for the whole process but instead contains the blueprint for the process. The class models the process of transferring trade data from one format to another. This is its only responsibility, its only concern, and the only reason that this class should change. If the process itself changes, this class will change to reflect it. But if you decide you no longer want to retrieve data from a Stream, log on to the console, or store the trades in a database, this class remains as is.

As prescribed by the Stairway pattern (introduced in Chapter 2, “Dependencies and layering”), the interfaces that the TradeProcessor now depends on all live in a separate assembly. This ensures that neither the client nor the implementation assemblies reference each other. Separated into another assembly are the three classes that implement these interfaces, the StreamTradeDataProvider, SimpleTradeParser, and AdoNetTradeStorage classes. Note that there is a naming convention used for these classes. First, the prefixed I was removed from the interface name and replaced with the implementation-specific context that is required of the class. So StreamTradeDataProvider allows you to infer that it is an implementation of the ITradeDataProvider interface that retrieves its data from a Stream object. The AdoNetTradeStorage class uses ADO.NET to persist the trade data. I have prefixed the ITradeParser implementation with the word Simple to indicate that it has no dependency context.

All three of these implementations are able to live in a single assembly due to their shared dependencies—core assemblies of the Microsoft .NET Framework. If you were to introduce an implementation that required a third-party dependency, a first-party dependency of your own, or a dependency from a non-core .NET Framework class, you should put these implementations into their own assemblies. For example, if you were to use the Dapper mapping library instead of ADO.NET, you would create an assembly called Services.Dapper, inside of which would be an ITradeStorage implementation called DapperTradeStorage.

The ITradeDataProvider interface does not depend on the Stream class. The previous version of the method for retrieving trade data required a Stream instance as a parameter, but this artificially tied the method to a dependency. When you are creating interfaces and refactoring toward abstractions, it is important that you do not retain dependencies where doing so would affect the adaptability of the code. The possibility of retrieving the trade data from sources other than a Stream has already been discussed, so the refactoring has ensured that this dependency is removed from the interface. Instead, the StreamTradeDataProvider requires a Stream as a constructor parameter, instead of a method parameter. By using the constructor, you can depend on almost anything without polluting the interface. Listing 5-10 shows the StreamTradeDataProvider implementation.

LISTING 5-10 Context can be passed into classes via constructor parameters, keeping the interface clean.

Click here to view code image

public class StreamTradeDataProvider : ITradeDataProvider
{
    public StreamTradeDataProvider(Stream stream)
    {
        this.stream = stream;
    }

    public IEnumerable<string> GetTradeData()
    {
        var tradeData = new List<string>();
        using (var reader = new StreamReader(stream))
        {
            string line;
            while ((line = reader.ReadLine()) != null)
            {
                tradeData.Add(line);
            }
        }
        return tradeData;
    }

    private Stream stream;
}

Remember that the TradeProcessor class, which is the client of this code, is aware of nothing other than the GetTradeData method’s signature via the ITradeDataProvider. It has no knowledge whatsoever of how the real implementation retrieves the data—nor should it.

There are more abstractions that can be extracted from this example. Remember that the original ParseTrades method delegated responsibility for validation and for mapping. You can repeat the process of refactoring so that the SimpleTradeParser class does not have more than one responsibility. Figure 5-2 shows in Unified Markup Language (UML) how this can be achieved.
Image

FIGURE 5-2 The SimpleTradeParser is also refactored to ensure that each class has a single responsibility.

This process of abstracting responsibilities into interfaces (and their accompanying implementations) is recursive. As you inspect each class, you must determine the responsibilities that it has and factor them out until the class has only one. Listing 5-11 shows the SimpleTradeParser class, which delegates to interfaces where appropriate. Its single reason for change is if the overall structure of the trade data changes—for instance, if the data no longer uses comma-separated values and changes to using tabs, or perhaps XML.

LISTING 5-11 The algorithm for parsing trade data is encapsulated in ITradeParser implementations.

Click here to view code image

public class SimpleTradeParser : ITradeParser
{
    public SimpleTradeParser(ITradeValidator tradeValidator, ITradeMapper tradeMapper)
    {
        this.tradeValidator = tradeValidator;
        this.tradeMapper = tradeMapper;
    }

    public IEnumerable<TradeRecord> Parse(IEnumerable<string> tradeData)
    {
        var trades = new List<TradeRecord>();
        var lineCount = 1;
        foreach (var line in tradeData)
        {
            var fields = line.Split(new char[] { ',' });

            if (!tradeValidator.Validate(fields))
            {
                continue;
            }

            var trade = tradeMapper.Map(fields);

            trades.Add(trade);

            lineCount++;
        }
        return trades;
    }

    private readonly ITradeValidator tradeValidator;
    private readonly ITradeMapper tradeMapper;
}

The final refactor aims to abstract logging from two classes. Both the ITradeValidator and ITradeStorage implementations are still logging directly to the console. This time, instead of implementing your own logging class, you will create an adapter for the popular logging library, Log4Net. The UML class diagram in Figure 5-3 shows how this all fits together.
Image

FIGURE 5-3 By implementing an adapter for Log4Net, you need not reference it in every assembly.

The net benefit of creating an adapter class such as Log4NetLoggerAdapter is that you can convert a third-party reference into a first-party reference. Notice that both AdoNetTradeStorage and SimpleTradeValidator both depend on the first-party ILogger interface. But, at run time, both will actually use Log4Net. The only references needed to Log4Net are in the entry point of the application (see Chapter 9, “Dependency injection,” for more information) and the newly created Service.Log4Net assembly. Any code that has a dependency on Log4Net, such as custom appenders, should live in the Service.Log4Net assembly. For now, only the adapter resides in this new assembly.

The refactored validator class is shown in Listing 5-12. It now has no reference whatsoever to the console. Because of Log4Net’s flexibility, you can actually log to almost anywhere now. Total adaptability has been achieved as far as logging is concerned.

LISTING 5-12 The SimpleTradeValidator class after refactoring.

Click here to view code image

public class SimpleTradeValidator : ITradeValidator
{
    private readonly ILogger logger;    public SimpleTradeValidator(ILogger logger)
    {
        this.logger = logger;
    }

    public bool Validate(string[] tradeData)
    {
        if (tradeData.Length != 3)
        {
            logger.LogWarning("Line malformed. Only {1} field(s) found.",
  tradeData.Length);
            return false;
        }

        if (tradeData[0].Length != 6)
        {
            logger.LogWarning("Trade currencies malformed: '{1}'", tradeData[0]);
            return false;
        }

        int tradeAmount;
        if (!int.TryParse(tradeData[1], out tradeAmount))
        {
            logger.LogWarning("Trade amount not a valid integer: '{1}'", tradeData[1]);
            return false;
        }

        decimal tradePrice;
        if (!decimal.TryParse(tradeData[2], out tradePrice))
        {
            logger.LogWarning("WARN: Trade price not a valid decimal: '{1}'",
  tradeData[2]);
            return false;
        }

        return true;
    }
}

At this point, a quick recap is in order. Bear in mind that you have altered nothing as far as the functionality of the code is concerned. Functionally, this code does exactly what it used to do. However, if you wanted to enhance it in any way, you could do so with ease. The added ability to adapt this code to a new purpose more than justifies the effort expended to refactor it.

Referring back to the original list of potential enhancements to this code, this new version allows you to implement each one without touching the existing classes.

Image Request: You decide not to use a Stream for input but instead read the trades from a remote call to a web service.

• Solution: Create a new ITradeDataProvider implementation that supplies the data from the service.

Image Request: The format of the input data changes, perhaps with the addition of an extra field indicating the broker for the transaction.

• Solution: Alter the implementations for the ITradeDataValidator, ITradeDataMapper, and ITradeStorage interfaces, which handle the new broker field.

Image Request: The validation rules of the input data change.

• Solution: Edit the ITradeDataValidator implementation to reflect the rule changes.

Image Request: The way in which you log warnings, errors, and information changes. If you are using a hosted web service, writing to the console would not be a viable option.

• Solution: As discussed, Log4Net provides you with infinite options for logging, by virtue of the adapter.

Image Request: The database changes in some way—perhaps the insert_trade stored procedure requires a new parameter for the broker, too, or you decide not to store the data in a relational database and opt for document storage, or the database is moved behind a web service that you must call.

• Solution: If the stored procedure changes, you would need to edit the AdoNetTradeStorage class to include the broker field. For the other two options, you could create a MongoTradeStorage class that uses MongoDB to store the trades, and you could create a ServiceTradeStorage class to hide the implementation behind a web service.

I hope you are now fully convinced that a combination of abstracting via interfaces, decoupling assemblies to follow the Stairway pattern, aggressive refactoring, and adhering to the single responsibility principle are the foundation of adaptive code.

When you arrive at a scenario in which your code is neatly delegating to abstractions, the possibilities are endless. The rest of this chapter concentrates on other ways in which you can focus on a single responsibility per class.
SRP and the Decorator pattern

The Decorator pattern is excellent for ensuring that each class has a single responsibility. Classes can often do too many things without an obvious way of splitting the responsibilities into other classes. The responsibilities seem too closely linked.

The Decorator pattern’s basic premise is that each decorator class fulfills the contract of a type and also accepts one or more of those types as constructor parameters. This is beneficial because functionality can be added to an existing class that implements a certain interface, and the decorator also acts—unbeknownst to clients—as an implementation of the required interface. Figure 5-4 shows a UML diagram of the Decorator design pattern.
Image

FIGURE 5-4 A UML diagram showing an implementation of the Decorator pattern.

A simple example of the pattern is shown in Listing 5-13, which does not pertain to a specific use of the pattern but provides a canonical example.

LISTING 5-13 A template example of the decorator pattern.

Click here to view code image

public interface IComponent
{
    void Something();
}
// . . .
public class ConcreteComponent : IComponent
{
    public void Something()
    {

    }
}
// . . .
public class DecoratorComponent : IComponent
{
    public DecoratorComponent(IComponent decoratedComponent)
    {
        this.decoratedComponent = decoratedComponent;
    }

    public void Something()
    {
        SomethingElse();
        decoratedComponent.Something();
    }

    private void SomethingElse()
    {

    }

    private readonly IComponent decoratedComponent;
}
// . . .
class Program
{
    static IComponent component;

    static void Main(string[] args)
    {
        component = new DecoratorComponent(new ConcreteComponent());
        component.Something();
    }
}

Because a client accepts the interface shown in the listing as a method parameter, you can provide either the original, undecorated type to that client or you can provide the decorated version. Note that the client will be oblivious: it will not have to change depending on which version it is being provided.
The Composite pattern

The Composite pattern is a specialization of the Decorator pattern and is one of the more common uses of that pattern. A UML diagram describing the Composite pattern’s collaborators is shown in Figure 5-5.
Image

FIGURE 5-5 The Composite pattern closely resembles the Decorator pattern.

The Composite pattern’s purpose is to allow you to treat many instances of an interface as if they were just one instance. Therefore, clients can accept just one instance of an interface, but they can be implicitly provided with many instances, without requiring the client to change. Listing 5-14 shows a composite decorator in practice.

LISTING 5-14 The composite implementation of an interface.

Click here to view code image

public interface IComponent
{
    void Something();
}
// . . .
public class Leaf : IComponent
{
    public void Something()
    {

    }
}
// . . .
public class CompositeComponent : IComponent
{
    public CompositeComponent()
    {
        children = new List<IComponent>();
    }

    public void AddComponent(IComponent component)
    {
        children.Add(component);
    }

    public void RemoveComponent(IComponent component)
    {
        children.Remove(component);
    }

    public void Something()
    {
        foreach(var child in children)
        {
            child.Something();
        }
    }

    private ICollection<IComponent> children;
}
// . . .
class Program
{
    static void Main(string[] args)
    {
        var composite = new CompositeComponent();
        composite.AddComponent(new Leaf());
        composite.AddComponent(new Leaf());
        composite.AddComponent(new Leaf());

        component = composite;
        component.Something();
    }

    static IComponent component;
}

In the CompositeComponent class, there are methods for adding and removing other instances of the IComponent. These methods do not form part of the interface and are for clients of the CompositeComponent class, directly. Whichever factory method or class is tasked with creating instances of the CompositeComponent class will also have to create the decorated instances and pass them into the Add method; otherwise, the clients of the IComponent would have to change in order to cope with compositions.

Whenever the Something method is called by the IComponent clients, the list of composed instances is enumerated, and their respective Something is called. This is how you reroute the call to a single instance of IComponent—of type CompositeComponent—to many other types.

Each instance that you supply to the CompositeComponent class must implement the IComponent interface—and this is enforced by the compiler due to C#’s strong typing—but the instances need not all be of the same concrete type. Because of the advantages of polymorphism, you can treat all implementations of an interface as instances of that interface. In the example shown in Listing 5-15, the CompositeComponent instances provided are of different types, further enhancing this pattern’s utility.

LISTING 5-15 Instances provided to the composite can be of different types.

Click here to view code image

public class SecondTypeOfLeaf : IComponent
{
    public void Something()
    {

    }
}
// . . .
public class AThirdLeafType : IComponent
{
    public void Something()
    {

    }
}
// . . .
public void AlternativeComposite()
{
    var composite = new CompositeComponent();
    composite.AddComponent(new Leaf());
    composite.AddComponent(new SecondTypeOfLeaf());
    composite.AddComponent(new AThirdLeafType());

    component = composite;
    composite.Something();
}

Taking this pattern to its logical conclusion, you can even pass in one or more instances of the CompositeComponent interface to the Add method, forming a chain of composite instances in a hierarchical tree structure.

Where should the composite live?

Chapter 2 introduced the Entourage anti-pattern, which states that implementations should not live in the same assemblies as their interfaces. However, there is an exception to that rule: implementations whose dependencies are a subset of their interface’s dependencies.

Depending on how the composite is implemented, it is likely that no further dependencies will be introduced. If this is true, the assembly in which the interface resides could also include the composite implementation.

In Chapter 2, classes were shown to be modeled as object graphs. That theme continues here, to further demonstrate how the Composite pattern works. In Figure 5-6, the nodes of the graph represent object instances, and the edges represent method calls.
Image

FIGURE 5-6 The object graph notation helps to visualize the runtime structure of the program.
Predicate decorators

The predicate decorator is a useful construct for hiding the conditional execution of code from clients. Listing 5-16 shows an example.

LISTING 5-16 This client will only execute the Something method on even days of the month.

Click here to view code image

public class DateTester
{
    public bool TodayIsAnEvenDayOfTheMonth
    {
        get
        {
            return DateTime.Now.Day % 2 == 0;
        }
    }
}
// . . .
class PredicatedDecoratorExample
{
    public PredicatedDecoratorExample(IComponent component)
    {
        this.component = component;
    }

    public void Run()
    {
        DateTester dateTester = new DateTester();
        if (dateTester.TodayIsAnEvenDayOfTheMonth)
        {
            component.Something();
        }
    }

    private readonly IComponent component;
}

The presence of the DateTester class in this example is a dependency that does not belong in this class. The initial temptation is to alter the code toward that of Listing 5-17. However, that is only a partial solution.

LISTING 5-17 An improvement is to require the dependency to be passed into the class.

Click here to view code image

class PredicatedDecoratorExample
{
    public PredicatedDecoratorExample(IComponent component)
    {
        this.component = component;
    }

    public void Run(DateTester dateTester)
    {
        if (dateTester.TodayIsAnEvenDayOfTheMonth)
        {
            component.Something();
        }
    }

    private readonly IComponent component;
}

You now require a parameter of the Run method, breaking the client’s public interface and burdening its clients with providing an implementation of the DateTester class. By using the Decorator pattern, you are able to keep the client’s interface the same, yet retain the conditional-execution functionality. Listing 5-18 proves that this is not too good to be true.

LISTING 5-18 The predicate decoration contains the dependency, and the client is much cleaner.

Click here to view code image

public class PredicatedComponent : IComponent
{
    public PredicatedComponent(IComponent decoratedComponent, DateTester dateTester)
    {
        this.decoratedComponent = decoratedComponent;
        this.dateTester = dateTester;
    }

    public void Something()
    {
        if(dateTester.TodayIsAnEvenDayOfTheMonth)
        {
            decoratedComponent.Something();
        }
    }

    private readonly IComponent decoratedComponent;
    private readonly DateTester dateTester;
}
// . . .
class PredicatedDecoratorExample
{
    public PredicatedDecoratorExample(IComponent component)
    {
        this.component = component;
    }

    public void Run()
    {
        component.Something();
    }

    private readonly IComponent component;
}

Note that this listing has added conditional branching to the code without modifying either the client code or the original implementing class. Also, this example has accepted the DateTester class as a dependency, but you could take this one step further by defining your own predicate interface for handling this scenario generically. After a few changes, the code looks like Listing 5-19.

LISTING 5-19 Defining a dedicated IPredicate interface makes the solution more general.

Click here to view code image

