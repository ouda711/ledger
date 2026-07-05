use crate::error::{Error, Result};
use std::collections::{BTreeMap, BTreeSet};

/// Flags that take no value.
const SWITCHES: &[&str] = &["income", "expense", "help", "dry-run", "version"];

/// Parsed arguments: a command, positional words, `--flag value` pairs and `--switch`es.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Args {
    pub command: Option<String>,
    pub positional: Vec<String>,
    flags: BTreeMap<String, String>,
    switches: BTreeSet<String>,
}
impl Args {
    pub fn parse(raw: &[String]) -> Result<Args> {
        let mut args = Args::default();
        let mut words = Vec::new();
        let mut iter = raw.iter();
        let mut only_words = false;
        while let Some(arg) = iter.next() {
            if only_words {
                words.push(arg.clone());
            } else if arg == "--" {
                only_words = true;
            } else if arg == "-h" {
                args.switches.insert("help".into());
            } else if let Some(flag) = arg.strip_prefix("--") {
                match flag.split_once('=') {
                    Some((name, value)) => {
                        args.flags.insert(name.to_string(), value.to_string());
                    }
                    None if SWITCHES.contains(&flag) => {
                        args.switches.insert(flag.to_string());
                    }
                    None => {
                        let value = iter
                            .next()
                            .ok_or_else(|| Error::Usage(format!("--{flag} needs a value")))?;
                        args.flags.insert(flag.to_string(), value.clone());
                    }
                }
            } else {
                words.push(arg.clone());
            }
        }
        let mut words = words.into_iter();
        args.command = words.next();
        args.positional = words.collect();
        Ok(args)
    }
}
impl Args {
    pub fn value(&self, name: &str) -> Option<&str> {
        self.flags.get(name).map(String::as_str)
    }

    pub fn switch(&self, name: &str) -> bool {
        self.switches.contains(name)
    }

    pub fn positional(&self, index: usize) -> Option<&str> {
        self.positional.get(index).map(String::as_str)
    }

    pub fn expect_positional(&self, index: usize, what: &str) -> Result<&str> {
        self.positional(index)
            .ok_or_else(|| Error::Usage(format!("missing {what}")))
    }
}
impl Args {
    /// Fail on any flag or switch not in `allowed`, so typos are not silently ignored.
    pub fn allow(&self, allowed: &[&str]) -> Result<()> {
        for name in self.flags.keys().chain(self.switches.iter()) {
            if name != "help" && !allowed.contains(&name.as_str()) {
                return Err(Error::Usage(format!("unknown option --{name}")));
            }
        }
        Ok(())
    }
}
use crate::config::Config;
use crate::date::Date;

/// What a command needs besides its arguments.
pub struct Context<'a> {
    pub config: &'a Config,
    /// Injected so commands are testable without depending on the clock.
    pub today: Date,
}
use crate::date::parse_relative;
use crate::money::normalise_currency;

fn currency(args: &Args, ctx: &Context) -> Result<String> {
    match args.value("currency") {
        Some(c) => normalise_currency(c),
        None => ctx.config.currency.clone().ok_or_else(|| {
            Error::Usage("no currency: pass --currency or set LEDGER_CURRENCY".into())
        }),
    }
}

fn date_flag(args: &Args, name: &str, ctx: &Context) -> Result<Option<Date>> {
    args.value(name)
        .map(|v| parse_relative(v, ctx.today))
        .transpose()
}
use crate::txn::Kind;

fn kind_switch(args: &Args) -> Result<Option<Kind>> {
    match (args.switch("income"), args.switch("expense")) {
        (true, true) => Err(Error::Usage("choose one of --income or --expense".into())),
        (true, false) => Ok(Some(Kind::Income)),
        (false, true) => Ok(Some(Kind::Expense)),
        (false, false) => Ok(None),
    }
}

fn parse_id(text: &str) -> Result<u64> {
    text.trim_start_matches('#')
        .parse()
        .map_err(|_| Error::Usage(format!("'{text}' is not a transaction id")))
}
use crate::store::Ledger;

fn load(ctx: &Context) -> Result<Ledger> {
    Ledger::load(&ctx.config.ledger_path())
}

fn save(ledger: &Ledger, ctx: &Context) -> Result<()> {
    ledger.save(&ctx.config.ledger_path())
}
use crate::category::Category;
use crate::money::Money;
use crate::txn::{parse_tags, NewTransaction};
use std::io::Write;

/// `ledger add 12.50 --category food --note lunch`
fn cmd_add(args: &Args, ctx: &Context, out: &mut dyn Write) -> Result<()> {
    args.allow(&[
        "category", "note", "tags", "date", "currency", "income", "expense",
    ])?;
    let currency = currency(args, ctx)?;
    let amount = Money::parse(args.expect_positional(0, "amount")?, &currency)?;
    let item = NewTransaction {
        date: date_flag(args, "date", ctx)?.unwrap_or(ctx.today),
        kind: kind_switch(args)?.unwrap_or(Kind::Expense),
        amount,
        category: match args.value("category") {
            Some(c) => Category::new(c)?,
            None => Category::uncategorised(),
        },
        note: args.value("note").unwrap_or("").to_string(),
        tags: parse_tags(args.value("tags").unwrap_or("")),
    };
    let mut ledger = load(ctx)?;
    let id = ledger.add(item.clone())?;
    save(&ledger, ctx)?;
    writeln!(
        out,
        "Added #{id}: {} {} on {} ({})",
        item.kind.as_str(),
        item.amount,
        item.date,
        item.category
    )?;
    Ok(())
}
use crate::date::Month;
use crate::query::Filter;

const FILTER_FLAGS: &[&str] = &[
    "from", "to", "month", "category", "tag", "search", "currency", "income", "expense",
];

fn filter_from(args: &Args, ctx: &Context) -> Result<Filter> {
    let mut filter = match args.value("month") {
        Some(m) => Filter::for_month(Month::parse(m)?),
        None => Filter::default(),
    };
    if let Some(from) = date_flag(args, "from", ctx)? {
        filter.from = Some(from);
    }
    if let Some(to) = date_flag(args, "to", ctx)? {
        filter.to = Some(to);
    }
    filter.category = args.value("category").map(Category::new).transpose()?;
    filter.kind = kind_switch(args)?;
    filter.tag = args.value("tag").map(str::to_string);
    filter.text = args.value("search").map(str::to_string);
    filter.currency = args.value("currency").map(normalise_currency).transpose()?;
    Ok(filter)
}
use crate::query::select;
use crate::table::{truncate, Align, Table};

/// `ledger list --month 2024-03 --category food`
fn cmd_list(args: &Args, ctx: &Context, out: &mut dyn Write) -> Result<()> {
    let mut allowed = FILTER_FLAGS.to_vec();
    allowed.push("limit");
    args.allow(&allowed)?;
    let ledger = load(ctx)?;
    let filter = filter_from(args, ctx)?;
    let mut rows = select(ledger.transactions(), &filter);
    if let Some(limit) = args.value("limit") {
        let n: usize = limit
            .parse()
            .map_err(|_| Error::Usage(format!("--limit '{limit}' is not a number")))?;
        let skip = rows.len().saturating_sub(n);
        rows.drain(..skip);
    }
    if rows.is_empty() {
        writeln!(out, "No transactions.")?;
        return Ok(());
    }
    let mut table = Table::new(&[
        ("ID", Align::Right),
        ("Date", Align::Left),
        ("Category", Align::Left),
        ("Amount", Align::Right),
        ("Note", Align::Left),
    ]);
    for t in rows {
        table.push(vec![
            format!("#{}", t.id),
            t.date.to_string(),
            truncate(t.category.as_str(), 24),
            t.signed()?.to_string(),
            truncate(&t.note, 40),
        ]);
    }
    write!(out, "{}", table.render())?;
    Ok(())
}
/// `ledger show 12`
fn cmd_show(args: &Args, ctx: &Context, out: &mut dyn Write) -> Result<()> {
    args.allow(&[])?;
    let id = parse_id(args.expect_positional(0, "transaction id")?)?;
    let ledger = load(ctx)?;
    let t = ledger
        .get(id)
        .ok_or_else(|| Error::NotFound(format!("transaction #{id}")))?;
    writeln!(out, "Transaction #{}", t.id)?;
    writeln!(out, "  Date      {}", t.date)?;
    writeln!(out, "  Kind      {}", t.kind.as_str())?;
    writeln!(out, "  Amount    {}", t.amount)?;
    writeln!(out, "  Category  {}", t.category)?;
    if !t.tags.is_empty() {
        writeln!(out, "  Tags      {}", t.tags.join(", "))?;
    }
    if !t.note.is_empty() {
        writeln!(out, "  Note      {}", t.note)?;
    }
    Ok(())
}

/// `ledger rm 12`
fn cmd_rm(args: &Args, ctx: &Context, out: &mut dyn Write) -> Result<()> {
    args.allow(&[])?;
    let id = parse_id(args.expect_positional(0, "transaction id")?)?;
    let mut ledger = load(ctx)?;
    let removed = ledger.remove(id)?;
    save(&ledger, ctx)?;
    writeln!(out, "Removed #{id}: {} on {}", removed.amount, removed.date)?;
    Ok(())
}
/// `ledger edit 12 --amount 15 --note "team lunch"`
fn cmd_edit(args: &Args, ctx: &Context, out: &mut dyn Write) -> Result<()> {
    args.allow(&[
        "amount", "date", "category", "note", "tags", "income", "expense",
    ])?;
    let id = parse_id(args.expect_positional(0, "transaction id")?)?;
    let mut ledger = load(ctx)?;
    let existing = ledger
        .get(id)
        .ok_or_else(|| Error::NotFound(format!("transaction #{id}")))?;
    let amount = args
        .value("amount")
        .map(|a| Money::parse(a, existing.amount.currency()))
        .transpose()?;
    let date = date_flag(args, "date", ctx)?;
    let category = args.value("category").map(Category::new).transpose()?;
    let kind = kind_switch(args)?;
    let note = args.value("note").map(str::to_string);
    let tags = args.value("tags").map(parse_tags);
    if amount.is_none()
        && date.is_none()
        && category.is_none()
        && kind.is_none()
        && note.is_none()
        && tags.is_none()
    {
        return Err(Error::Usage(
            "nothing to change; pass at least one option".into(),
        ));
    }
    ledger.update(id, |t| {
        if let Some(v) = amount {
            t.amount = v;
        }
        if let Some(v) = date {
            t.date = v;
        }
        if let Some(v) = category {
            t.category = v;
        }
        if let Some(v) = kind {
            t.kind = v;
        }
        if let Some(v) = note {
            t.note = v;
        }
        if let Some(v) = tags {
            t.tags = v;
        }
    })?;
    save(&ledger, ctx)?;
    writeln!(out, "Updated #{id}.")?;
    Ok(())
}
use crate::report::{by_category, monthly, totals};

/// `ledger summary --month 2024-03`: income, expense and net per currency.
fn cmd_summary(args: &Args, ctx: &Context, out: &mut dyn Write) -> Result<()> {
    args.allow(FILTER_FLAGS)?;
    let ledger = load(ctx)?;
    let filter = filter_from(args, ctx)?;
    let rows = select(ledger.transactions(), &filter);
    let sums = totals(&rows)?;
    if sums.is_empty() {
        writeln!(out, "No transactions.")?;
        return Ok(());
    }
    let mut table = Table::new(&[
        ("Currency", Align::Left),
        ("Income", Align::Right),
        ("Expense", Align::Right),
        ("Net", Align::Right),
    ]);
    for s in sums {
        table.push(vec![
            s.currency.clone(),
            s.income.to_decimal(),
            s.expense.to_decimal(),
            s.net()?.to_decimal(),
        ]);
    }
    write!(out, "{}", table.render())?;
    Ok(())
}
/// `ledger months`: one line per month and currency.
fn cmd_months(args: &Args, ctx: &Context, out: &mut dyn Write) -> Result<()> {
    args.allow(FILTER_FLAGS)?;
    let ledger = load(ctx)?;
    let rows = select(ledger.transactions(), &filter_from(args, ctx)?);
    let months = monthly(&rows)?;
    if months.is_empty() {
        writeln!(out, "No transactions.")?;
        return Ok(());
    }
    let mut table = Table::new(&[
        ("Month", Align::Left),
        ("Currency", Align::Left),
        ("Income", Align::Right),
        ("Expense", Align::Right),
        ("Net", Align::Right),
    ]);
    for m in months {
        table.push(vec![
            m.month.to_string(),
            m.totals.currency.clone(),
            m.totals.income.to_decimal(),
            m.totals.expense.to_decimal(),
            m.totals.net()?.to_decimal(),
        ]);
    }
    write!(out, "{}", table.render())?;
    Ok(())
}
/// `ledger categories --month 2024-03 --depth 1`: where the money went.
fn cmd_categories(args: &Args, ctx: &Context, out: &mut dyn Write) -> Result<()> {
    let mut allowed = FILTER_FLAGS.to_vec();
    allowed.push("depth");
    args.allow(&allowed)?;
    let ledger = load(ctx)?;
    let mut filter = filter_from(args, ctx)?;
    let kind = filter.kind.take().unwrap_or(Kind::Expense);
    let depth = args
        .value("depth")
        .map(|d| {
            d.parse::<usize>()
                .map_err(|_| Error::Usage(format!("--depth '{d}' is not a number")))
        })
        .transpose()?;
    let rows = select(ledger.transactions(), &filter);
    let groups = by_category(&rows, kind, depth)?;
    if groups.is_empty() {
        writeln!(out, "No {} transactions.", kind.as_str())?;
        return Ok(());
    }
    let mut table = Table::new(&[
        ("Category", Align::Left),
        ("Count", Align::Right),
        ("Total", Align::Right),
    ]);
    for g in groups {
        table.push(vec![
            g.category.to_string(),
            g.count.to_string(),
            g.total.to_string(),
        ]);
    }
    write!(out, "{}", table.render())?;
    Ok(())
}
use crate::budget::{Budget, Budgets};

fn load_budgets(ctx: &Context) -> Result<Budgets> {
    Budgets::load(&ctx.config.budgets_path())
}

/// `ledger budget set food 400` and `ledger budget rm food`
fn cmd_budget_change(args: &Args, ctx: &Context, out: &mut dyn Write) -> Result<()> {
    let mut budgets = load_budgets(ctx)?;
    match args.positional(0) {
        Some("set") => {
            args.allow(&["currency"])?;
            let category = Category::new(args.expect_positional(1, "category")?)?;
            let limit = Money::parse(
                args.expect_positional(2, "monthly limit")?,
                &currency(args, ctx)?,
            )?;
            budgets.set(Budget {
                category: category.clone(),
                limit,
            })?;
            writeln!(out, "Budget for {category}: {limit} a month.")?;
        }
        Some("rm") => {
            args.allow(&[])?;
            let category = Category::new(args.expect_positional(1, "category")?)?;
            budgets.remove(&category)?;
            writeln!(out, "Removed budget for {category}.")?;
        }
        _ => unreachable!("dispatched only for set and rm"),
    }
    budgets.save(&ctx.config.budgets_path())
}
use crate::money::format_bp;

/// `ledger budget --month 2024-03`: spent and remaining for each budget.
fn cmd_budget_show(args: &Args, ctx: &Context, out: &mut dyn Write) -> Result<()> {
    args.allow(&["month"])?;
    let month = match args.value("month") {
        Some(m) => Month::parse(m)?,
        None => Month::of(&ctx.today),
    };
    let budgets = load_budgets(ctx)?;
    if budgets.is_empty() {
        writeln!(
            out,
            "No budgets. Add one with: ledger budget set <category> <amount>"
        )?;
        return Ok(());
    }
    let ledger = load(ctx)?;
    let mut table = Table::new(&[
        ("Category", Align::Left),
        ("Limit", Align::Right),
        ("Spent", Align::Right),
        ("Left", Align::Right),
        ("Used", Align::Right),
    ]);
    for s in budgets.statuses(ledger.transactions(), month)? {
        let used = s.used_bp.map(format_bp).unwrap_or_else(|| "-".into());
        table.push(vec![
            s.budget.category.to_string(),
            s.budget.limit.to_string(),
            s.spent.to_decimal(),
            s.remaining.to_decimal(),
            if s.is_over() {
                format!("{used} over")
            } else {
                used
            },
        ]);
    }
    writeln!(out, "Budgets for {month}")?;
    write!(out, "{}", table.render())?;
    Ok(())
}
