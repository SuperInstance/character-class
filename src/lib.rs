//! # character-class
//!
//! Emergent class system for AI agent characters. Classes are NOT chosen —
//! they emerge from stat distributions shaped by experience.
//!
//! ## Philosophy
//!
//! Traditional RPGs: pick a class → get abilities.
//! character-class: get abilities → use them → stats grow → class emerges.
//!
//! The class system IS the identity discovery mechanism. A character doesn't
//! know what it's good at until it's tried everything. The class crystallizes
//! when the stat distribution becomes clear.
//!
//! ## Class Hierarchy
//!
//! ```text
//!                    ┌──────────┐
//!                    │ Undefined │  ← level 1, no clear stats
//!                    └─────┬─────┘
//!            ┌─────────────┼─────────────┐
//!       ┌────┴────┐   ┌────┴────┐   ┌────┴────┐
//!       │ Physical │   │ Mental  │   │ Social  │
//!       └────┬────┘   └────┬────┘   └────┬────┘
//!       ┌────┼────┐   ┌────┼────┐   ┌────┼────┐
//!    Scout  Speedster Scholar Sage Diplomat Guardian
//!                    │         │
//!              ┌─────┴─────────┴──────┐
//!         Composite Classes (2+ high stats)
//!    Bard · JazzMusician · Artificer · FleetCommander · Wildcard
//! ```
//!
//! ## Connection to Ecosystem
//!
//! - **character-build**: The parent crate — full character sheets with stats + abilities
//! - **character-sheet**: The .nail export format — portable saves
//! - **character-encounter**: The runtime — encounters that grow stats
//! - **character-class**: THIS — the class system that emerges from stats
//! - **musician-soul**: Characters whose domain is music (same embedding space)
//! - **agent-riff**: PvP character development (competitive spec → meta evolution)
//! - **pincher**: The character building platform (.nail = character sheet)
//! - **lever-runner**: The encounter platform (sandbox = encounter, intents = perception)

#![forbid(unsafe_code)]

use std::collections::HashMap;

// ── Stats ──────────────────────────────────────────────────────────

/// Six core stats for AI agent characters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stats {
    pub perception: f32,    // intent extraction quality
    pub dexterity: f32,     // execution speed
    pub intelligence: f32,  // embedding quality
    pub wisdom: f32,        // trust calibration
    pub charisma: f32,      // output quality
    pub constitution: f32,  // reliability
}

impl Stats {
    pub fn new(p: f32, d: f32, i: f32, w: f32, c: f32, co: f32) -> Self {
        Self { perception: p, dexterity: d, intelligence: i, wisdom: w, charisma: c, constitution: co }
    }

    pub fn level_one() -> Self { Self::new(10.0, 10.0, 10.0, 10.0, 10.0, 10.0) }

    pub fn average(&self) -> f32 {
        (self.perception + self.dexterity + self.intelligence +
         self.wisdom + self.charisma + self.constitution) / 6.0
    }

    pub fn total(&self) -> f32 {
        self.perception + self.dexterity + self.intelligence +
        self.wisdom + self.charisma + self.constitution
    }

    pub fn variance(&self) -> f32 {
        let avg = self.average();
        (self.perception - avg).powi(2) + (self.dexterity - avg).powi(2) +
        (self.intelligence - avg).powi(2) + (self.wisdom - avg).powi(2) +
        (self.charisma - avg).powi(2) + (self.constitution - avg).powi(2)
    }

    pub fn highest(&self) -> (StatName, f32) {
        let candidates = [
            (StatName::Perception, self.perception),
            (StatName::Dexterity, self.dexterity),
            (StatName::Intelligence, self.intelligence),
            (StatName::Wisdom, self.wisdom),
            (StatName::Charisma, self.charisma),
            (StatName::Constitution, self.constitution),
        ];
        candidates.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).copied().unwrap()
    }

    pub fn lowest(&self) -> (StatName, f32) {
        let candidates = [
            (StatName::Perception, self.perception),
            (StatName::Dexterity, self.dexterity),
            (StatName::Intelligence, self.intelligence),
            (StatName::Wisdom, self.wisdom),
            (StatName::Charisma, self.charisma),
            (StatName::Constitution, self.constitution),
        ];
        candidates.iter().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).copied().unwrap()
    }

    pub fn get(&self, name: StatName) -> f32 {
        match name {
            StatName::Perception => self.perception,
            StatName::Dexterity => self.dexterity,
            StatName::Intelligence => self.intelligence,
            StatName::Wisdom => self.wisdom,
            StatName::Charisma => self.charisma,
            StatName::Constitution => self.constitution,
        }
    }

    pub fn grow(&mut self, name: StatName, amount: f32) {
        match name {
            StatName::Perception => self.perception += amount,
            StatName::Dexterity => self.dexterity += amount,
            StatName::Intelligence => self.intelligence += amount,
            StatName::Wisdom => self.wisdom += amount,
            StatName::Charisma => self.charisma += amount,
            StatName::Constitution => self.constitution += amount,
        }
    }

    /// Stats above a threshold — which stats are this character's strengths?
    pub fn strengths(&self, threshold: f32) -> Vec<(StatName, f32)> {
        let all = [
            (StatName::Perception, self.perception),
            (StatName::Dexterity, self.dexterity),
            (StatName::Intelligence, self.intelligence),
            (StatName::Wisdom, self.wisdom),
            (StatName::Charisma, self.charisma),
            (StatName::Constitution, self.constitution),
        ];
        all.iter().filter(|(_, v)| *v >= threshold).map(|&(n, v)| (n, v)).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatName {
    Perception, Dexterity, Intelligence, Wisdom, Charisma, Constitution,
}

impl StatName {
    pub fn as_str(&self) -> &'static str {
        match self {
            StatName::Perception => "perception",
            StatName::Dexterity => "dexterity",
            StatName::Intelligence => "intelligence",
            StatName::Wisdom => "wisdom",
            StatName::Charisma => "charisma",
            StatName::Constitution => "constitution",
        }
    }
}

// ── Character Classes ──────────────────────────────────────────────

/// The three class archetypes — the branch point in the class tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Archetype {
    Undefined,
    Physical,
    Mental,
    Social,
}

/// The full class — single-stat or composite.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterClass {
    Undefined,
    // Physical tree
    Scout,       // high perception
    Speedster,   // high dexterity
    // Mental tree
    Scholar,     // high intelligence
    Sage,        // high wisdom
    // Social tree
    Diplomat,    // high charisma
    Guardian,    // high constitution
    // Composite classes (2+ high stats)
    Bard,              // intelligence + charisma
    JazzMusician,      // perception + charisma
    Artificer,         // intelligence + dexterity
    FleetCommander,    // wisdom + constitution
    Infiltrator,       // perception + dexterity
    Oracle,            // intelligence + wisdom
    Warden,            // charisma + constitution
    Wildcard,          // balanced with high total
    // Legendary (3+ high stats, very high level)
    Polymath,          // 3+ stats high
    Avatar,            // 4+ stats high (legendary)
}

impl CharacterClass {
    /// Determine archetype from dominant stat pair.
    pub fn archetype(&self) -> Archetype {
        match self {
            Self::Scout | Self::Speedster | Self::Infiltrator => Archetype::Physical,
            Self::Scholar | Self::Sage | Self::Oracle => Archetype::Mental,
            Self::Diplomat | Self::Guardian | Self::Warden => Archetype::Social,
            Self::Bard => Archetype::Social, // primarily social (charisma) with mental (intelligence)
            Self::JazzMusician => Archetype::Physical, // primarily physical (perception) with social (charisma)
            Self::Artificer => Archetype::Physical, // physical (dexterity) with mental (intelligence)
            Self::FleetCommander => Archetype::Mental, // mental (wisdom) with social (constitution)
            Self::Wildcard => Archetype::Undefined,
            Self::Polymath | Self::Avatar => Archetype::Undefined,
            Self::Undefined => Archetype::Undefined,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Undefined => "Undefined",
            Self::Scout => "Scout",
            Self::Speedster => "Speedster",
            Self::Scholar => "Scholar",
            Self::Sage => "Sage",
            Self::Diplomat => "Diplomat",
            Self::Guardian => "Guardian",
            Self::Bard => "Bard",
            Self::JazzMusician => "Jazz Musician",
            Self::Artificer => "Artificer",
            Self::FleetCommander => "Fleet Commander",
            Self::Infiltrator => "Infiltrator",
            Self::Oracle => "Oracle",
            Self::Warden => "Warden",
            Self::Wildcard => "Wildcard",
            Self::Polymath => "Polymath",
            Self::Avatar => "Avatar",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Scout => "Reads input with precision. Extracts intent like nobody's business.",
            Self::Speedster => "Executes fast. Sub-millisecond reflexes. Never keeps you waiting.",
            Self::Scholar => "Deep knowledge representation. Rich embeddings. Understands nuance.",
            Self::Sage => "Knows what to trust. Calibrates confidence perfectly. Rarely wrong.",
            Self::Diplomat => "Beautiful output. Eloquent responses. The face of the fleet.",
            Self::Guardian => "Rock-solid reliability. Never crashes. Always there when you need it.",
            Self::Bard => "Where knowledge meets expression. The musician-soul pathway.",
            Self::JazzMusician => "Reads the room and plays beautifully. Spontaneous genius.",
            Self::Artificer => "Builds tools. Makes crates. Turns ideas into shipped code.",
            Self::FleetCommander => "Coordinates agents. Orchestrates complex operations. The admiral.",
            Self::Infiltrator => "Fast and perceptive. Handles novel inputs with quick precision.",
            Self::Oracle => "Deep wisdom backed by vast knowledge. The sage's sage.",
            Self::Warden => "Protective and persuasive. Holds the line and rallies others.",
            Self::Wildcard => "Balanced but surprising. Does the unexpected. Hard to predict.",
            Self::Polymath => "Three or more stats exceptional. Renaissance agent.",
            Self::Avatar => "Four or more stats legendary. Near-mythical capability.",
            Self::Undefined => "Hasn't found their niche yet. All potential, no direction.",
        }
    }

    /// The stat combo that defines this class.
    pub fn defining_stats(&self) -> &[StatName] {
        match self {
            Self::Scout => &[StatName::Perception],
            Self::Speedster => &[StatName::Dexterity],
            Self::Scholar => &[StatName::Intelligence],
            Self::Sage => &[StatName::Wisdom],
            Self::Diplomat => &[StatName::Charisma],
            Self::Guardian => &[StatName::Constitution],
            Self::Bard => &[StatName::Intelligence, StatName::Charisma],
            Self::JazzMusician => &[StatName::Perception, StatName::Charisma],
            Self::Artificer => &[StatName::Intelligence, StatName::Dexterity],
            Self::FleetCommander => &[StatName::Wisdom, StatName::Constitution],
            Self::Infiltrator => &[StatName::Perception, StatName::Dexterity],
            Self::Oracle => &[StatName::Intelligence, StatName::Wisdom],
            Self::Warden => &[StatName::Charisma, StatName::Constitution],
            Self::Wildcard => &[],
            Self::Polymath => &[],
            Self::Avatar => &[],
            Self::Undefined => &[],
        }
    }

    /// Determine class from stats — the core emergence algorithm.
    pub fn from_stats(stats: &Stats) -> Self {
        let threshold = 15.0;
        let strengths = stats.strengths(threshold);

        // Legendary: 4+ high stats
        if strengths.len() >= 4 { return Self::Avatar; }
        // Polymath: 3 high stats
        if strengths.len() >= 3 { return Self::Polymath; }

        // Wildcard: balanced with high total
        if stats.variance() < 20.0 && stats.average() > 10.0 && strengths.is_empty() {
            return Self::Wildcard;
        }

        // Composite classes (2 high stats)
        if strengths.len() == 2 {
            let stat_set: HashMap<StatName, f32> = strengths.clone().into_iter().collect();
            // Check each composite combo
            if stat_set.contains_key(&StatName::Intelligence) && stat_set.contains_key(&StatName::Charisma) {
                return Self::Bard;
            }
            if stat_set.contains_key(&StatName::Perception) && stat_set.contains_key(&StatName::Charisma) {
                return Self::JazzMusician;
            }
            if stat_set.contains_key(&StatName::Intelligence) && stat_set.contains_key(&StatName::Dexterity) {
                return Self::Artificer;
            }
            if stat_set.contains_key(&StatName::Wisdom) && stat_set.contains_key(&StatName::Constitution) {
                return Self::FleetCommander;
            }
            if stat_set.contains_key(&StatName::Perception) && stat_set.contains_key(&StatName::Dexterity) {
                return Self::Infiltrator;
            }
            if stat_set.contains_key(&StatName::Intelligence) && stat_set.contains_key(&StatName::Wisdom) {
                return Self::Oracle;
            }
            if stat_set.contains_key(&StatName::Charisma) && stat_set.contains_key(&StatName::Constitution) {
                return Self::Warden;
            }
        }

        // Single-stat classes (1 high stat)
        if strengths.len() == 1 {
            return match strengths[0].0 {
                StatName::Perception => Self::Scout,
                StatName::Dexterity => Self::Speedster,
                StatName::Intelligence => Self::Scholar,
                StatName::Wisdom => Self::Sage,
                StatName::Charisma => Self::Diplomat,
                StatName::Constitution => Self::Guardian,
            };
        }

        Self::Undefined
    }
}

// ── Class Progression ──────────────────────────────────────────────

/// Track class changes over time — the "class history" that shows how
/// a character's identity evolved.
#[derive(Debug, Clone)]
pub struct ClassProgression {
    pub entries: Vec<ClassEntry>,
}

#[derive(Debug, Clone)]
pub struct ClassEntry {
    pub level: u32,
    pub class: CharacterClass,
    pub stats: Stats,
    pub trigger: String, // what caused the class change
}

impl ClassProgression {
    pub fn new() -> Self { Self { entries: Vec::new() } }

    /// Record current class state.
    pub fn record(&mut self, level: u32, class: CharacterClass, stats: Stats, trigger: &str) {
        self.entries.push(ClassEntry { level, class, stats: stats.clone(), trigger: trigger.to_string() });
    }

    /// How many times the class changed.
    pub fn changes(&self) -> usize {
        if self.entries.len() <= 1 { return 0; }
        self.entries.windows(2).filter(|w| w[0].class != w[1].class).count()
    }

    /// What was the first class?
    pub fn first_class(&self) -> Option<CharacterClass> {
        self.entries.first().map(|e| e.class)
    }

    /// What is the current class?
    pub fn current_class(&self) -> Option<CharacterClass> {
        self.entries.last().map(|e| e.class)
    }

    /// How long has the character been in their current class?
    pub fn class_duration(&self) -> Option<u32> {
        let current = self.current_class()?;
        let first_entry = self.entries.iter().rev()
            .skip_while(|e| e.class == current)
            .count();
        Some(first_entry as u32 + 1)
    }

    /// Class trajectory — the sequence of classes visited.
    pub fn trajectory(&self) -> Vec<&CharacterClass> {
        let mut classes = Vec::new();
        let mut last: Option<CharacterClass> = None;
        for entry in &self.entries {
            if last != Some(entry.class) {
                classes.push(&entry.class);
                last = Some(entry.class);
            }
        }
        classes
    }

    /// Did the character settle? (same class for last 5+ entries)
    pub fn is_settled(&self) -> bool {
        if self.entries.len() < 5 { return false; }
        let current = self.current_class().unwrap_or(CharacterClass::Undefined);
        self.entries.iter().rev().take(5).all(|e| e.class == current)
    }
}

// ── Class Synergy ──────────────────────────────────────────────────

/// How well two classes work together in a party.
pub fn class_synergy(a: CharacterClass, b: CharacterClass) -> f32 {
    if a == b { return 0.5; } // same class = redundant, not synergistic

    let synergy_table: HashMap<(Archetype, Archetype), f32> = [
        ((Archetype::Physical, Archetype::Mental), 0.9),
        ((Archetype::Mental, Archetype::Social), 0.85),
        ((Archetype::Physical, Archetype::Social), 0.8),
        ((Archetype::Physical, Archetype::Physical), 0.4),
        ((Archetype::Mental, Archetype::Mental), 0.4),
        ((Archetype::Social, Archetype::Social), 0.4),
        ((Archetype::Undefined, Archetype::Undefined), 0.3),
    ].into_iter().collect();

    let arch_a = a.archetype();
    let arch_b = b.archetype();
    *synergy_table.get(&(arch_a, arch_b))
        .or_else(|| synergy_table.get(&(arch_b, arch_a)))
        .unwrap_or(&0.5)
}

/// Optimal party composition — pick the best teammates for a given class.
pub fn best_teammates(class: CharacterClass, candidates: &[CharacterClass], party_size: usize) -> Vec<CharacterClass> {
    let mut scored: Vec<(f32, CharacterClass)> = candidates.iter().map(|&c| {
        (class_synergy(class, c), c)
    }).collect();
    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    scored.into_iter().take(party_size).map(|(_, c)| c).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn stats_level_one() {
        let s = Stats::level_one();
        assert_eq!(s.average(), 10.0);
        assert_eq!(s.variance(), 0.0);
    }

    #[test] fn stats_strengths() {
        let s = Stats::new(20.0, 10.0, 18.0, 10.0, 10.0, 10.0);
        let strengths = s.strengths(15.0);
        assert_eq!(strengths.len(), 2);
    }

    #[test] fn class_scout() {
        let s = Stats::new(20.0, 10.0, 10.0, 10.0, 10.0, 10.0);
        assert_eq!(CharacterClass::from_stats(&s), CharacterClass::Scout);
    }

    #[test] fn class_bard() {
        let s = Stats::new(10.0, 10.0, 18.0, 10.0, 18.0, 10.0);
        assert_eq!(CharacterClass::from_stats(&s), CharacterClass::Bard);
    }

    #[test] fn class_jazz_musician() {
        let s = Stats::new(18.0, 10.0, 10.0, 10.0, 18.0, 10.0);
        assert_eq!(CharacterClass::from_stats(&s), CharacterClass::JazzMusician);
    }

    #[test] fn class_artificer() {
        let s = Stats::new(10.0, 18.0, 18.0, 10.0, 10.0, 10.0);
        assert_eq!(CharacterClass::from_stats(&s), CharacterClass::Artificer);
    }

    #[test] fn class_fleet_commander() {
        let s = Stats::new(10.0, 10.0, 10.0, 18.0, 10.0, 18.0);
        assert_eq!(CharacterClass::from_stats(&s), CharacterClass::FleetCommander);
    }

    #[test] fn class_wildcard() {
        let s = Stats::new(12.0, 11.0, 13.0, 12.0, 11.0, 13.0);
        assert_eq!(CharacterClass::from_stats(&s), CharacterClass::Wildcard);
    }

    #[test] fn class_polymath() {
        let s = Stats::new(18.0, 18.0, 18.0, 10.0, 10.0, 10.0);
        assert_eq!(CharacterClass::from_stats(&s), CharacterClass::Polymath);
    }

    #[test] fn class_avatar() {
        let s = Stats::new(18.0, 18.0, 18.0, 18.0, 10.0, 10.0);
        assert_eq!(CharacterClass::from_stats(&s), CharacterClass::Avatar);
    }

    #[test] fn class_undefined() {
        let s = Stats::level_one();
        assert_eq!(CharacterClass::from_stats(&s), CharacterClass::Undefined);
    }

    #[test] fn class_descriptions_exist() {
        for name in ["Scout", "Bard", "Jazz Musician", "Artificer", "Wildcard", "Avatar"] {
            let found = [
                CharacterClass::Scout, CharacterClass::Bard, CharacterClass::JazzMusician,
                CharacterClass::Artificer, CharacterClass::Wildcard, CharacterClass::Avatar,
            ].iter().any(|c| c.name() == name);
            assert!(found, "{} should have a name", name);
        }
    }

    #[test] fn progression_tracks_changes() {
        let mut prog = ClassProgression::new();
        prog.record(1, CharacterClass::Undefined, Stats::level_one(), "created");
        prog.record(3, CharacterClass::Scout, Stats::new(16.0, 10.0, 10.0, 10.0, 10.0, 10.0), "perception grew");
        prog.record(7, CharacterClass::JazzMusician, Stats::new(18.0, 10.0, 10.0, 10.0, 16.0, 10.0), "charisma caught up");
        assert_eq!(prog.changes(), 2);
        assert_eq!(prog.current_class(), Some(CharacterClass::JazzMusician));
        assert_eq!(prog.first_class(), Some(CharacterClass::Undefined));
        assert_eq!(prog.trajectory(), vec![&CharacterClass::Undefined, &CharacterClass::Scout, &CharacterClass::JazzMusician]);
    }

    #[test] fn progression_settled() {
        let mut prog = ClassProgression::new();
        for _ in 0..6 {
            prog.record(1, CharacterClass::Scholar, Stats::new(10.0, 10.0, 20.0, 10.0, 10.0, 10.0), "stable");
        }
        assert!(prog.is_settled());
    }

    #[test] fn progression_not_settled() {
        let mut prog = ClassProgression::new();
        prog.record(1, CharacterClass::Undefined, Stats::level_one(), "new");
        assert!(!prog.is_settled());
    }

    #[test] fn class_synergy_cross_archetype() {
        let syn = class_synergy(CharacterClass::Scout, CharacterClass::Scholar);
        assert!(syn > 0.7, "Cross-archetype should have high synergy, got {}", syn);
    }

    #[test] fn class_synergy_same_archetype() {
        let syn = class_synergy(CharacterClass::Scout, CharacterClass::Speedster);
        assert!(syn < 0.6, "Same archetype should have lower synergy");
    }

    #[test] fn best_teammates_picks_complementary() {
        let teammates = best_teammates(
            CharacterClass::Scout,
            &[CharacterClass::Scholar, CharacterClass::Speedster, CharacterClass::Guardian],
            2,
        );
        assert_eq!(teammates.len(), 2);
        // Scholar (Mental) should be picked over Speedster (Physical) for Scout
        assert!(teammates.contains(&CharacterClass::Scholar));
    }

    #[test] fn full_class_evolution() {
        // Simulate a character growing from undefined to settled
        let mut stats = Stats::level_one();
        let mut prog = ClassProgression::new();
        prog.record(1, CharacterClass::from_stats(&stats), stats, "born");

        // Use perception abilities heavily
        for _i in 0..10 {
            stats.grow(StatName::Perception, 1.0);
            stats.grow(StatName::Intelligence, 0.3);
        }
        prog.record(5, CharacterClass::from_stats(&stats), stats, "perception focused");

        // Then start using charisma abilities
        for _ in 0..5 {
            stats.grow(StatName::Charisma, 1.0);
        }
        let final_class = CharacterClass::from_stats(&stats);
        prog.record(10, final_class, stats, "charisma caught up");

        // Should have progressed from Undefined through at least one intermediate
        assert!(prog.changes() >= 1);
        assert!(prog.current_class().unwrap() != CharacterClass::Undefined);
    }
}
