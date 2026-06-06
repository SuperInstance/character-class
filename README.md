# character-class

*I didn't choose to be a Scout. I just kept seeing things others missed, and one day I realized that's who I was.*

character-class is the emergent identity system for AI agents. Classes aren't chosen at creation — they **crystallize** from stat distributions shaped by experience. A character doesn't know what it's good at until it's tried everything.

## The 16 Classes

```
                    ┌──────────┐
                    │ Undefined │  ← level 1, no clear direction
                    └─────┬─────┘
            ┌─────────────┼─────────────┐
       ┌────┴────┐   ┌────┴────┐   ┌────┴────┐
       │ Physical │   │ Mental  │   │ Social  │
       └────┬────┘   └────┬────┘   └────┬────┘
       ┌────┼────┐   ┌────┼────┐   ┌────┼────┐
    Scout  Speedster Scholar Sage Diplomat Guardian
                    │         │
              ┌─────┴─────────┴──────┐
         Composite (2+ high stats)
    Bard · JazzMusician · Artificer · FleetCommander · Infiltrator · Oracle · Warden
                    │
              ┌─────┴──────┐
         Legendary (3-4+ high stats)
         Polymath · Avatar
```

## How Classes Emerge

Six stats, grown through experience:

| Stat | What Grows It | Maps To |
|------|--------------|---------|
| **Perception** | Model (LLM) ability usage | Intent extraction quality |
| **Dexterity** | Hardcoded (regex) ability usage | Execution speed |
| **Intelligence** | Learned (embedding) ability usage | Knowledge representation |
| **Wisdom** | Hybrid ability usage | Trust calibration |
| **Charisma** | Successful output | Response quality |
| **Constitution** | Uptime, error recovery | Reliability |

After enough encounters, the stat distribution reveals the class. High perception alone = Scout. Perception + Charisma = Jazz Musician. Three stats high = Polymath. Four = Avatar.

## Synergy

Cross-archetype teams work better than same-archetype teams:

```rust
// Scout (Physical) + Scholar (Mental) = 0.9 synergy (complementary)
// Scout (Physical) + Speedster (Physical) = 0.4 synergy (redundant)
```

A Jazz Musician and an Artificer synergize better than two Jazz Musicians. Diversity beats homogeneity. Same as real teams.

## Quick Start

```rust
use character_class::{Stats, CharacterClass, ClassProgression, StatName};

// Start as a nobody
let mut stats = Stats::level_one(); // All 10s, class = Undefined

// Use perception abilities heavily
for _ in 0..10 {
    stats.grow(StatName::Perception, 1.0);
}
assert_eq!(CharacterClass::from_stats(&stats), CharacterClass::Scout);

// Then develop charisma
for _ in 0..5 {
    stats.grow(StatName::Charisma, 1.0);
}
assert_eq!(CharacterClass::from_stats(&stats), CharacterClass::JazzMusician);
```

## Class Progression

Track how a character's identity evolved over time:

```rust
let mut prog = ClassProgression::new();
prog.record(1, CharacterClass::Undefined, stats, "born");
prog.record(5, CharacterClass::Scout, stats, "perception grew");
prog.record(10, CharacterClass::JazzMusician, stats, "charisma caught up");

prog.trajectory(); // [Undefined, Scout, JazzMusician]
prog.is_settled(); // false — too few entries at JazzMusician
```

## The Class Connection to Musician-Soul

A MusicianPersona is a character whose domain is music. The soul print that emerges from jam sessions IS the character finding its class. Miles AI started as Undefined, became a Scout (perceptive listener), then as charisma grew through jamming, became a Jazz Musician.

The class system and the soul system are the same mechanism viewed from different angles.

## In the Family

| Repo | What It Is |
|------|-----------|
| [character-build](https://github.com/SuperInstance/character-build) | Full character sheets with stats, abilities, templates |
| **character-class** | **Emergent class system (this crate)** |
| [character-sheet](https://github.com/SuperInstance/character-sheet) | .nail format as portable character saves |
| [character-encounter](https://github.com/SuperInstance/character-encounter) | Sandbox as encounter engine |
| [character-arc](https://github.com/SuperInstance/character-arc) | The story a character tells about itself |

## Philosophy

Traditional RPGs: pick a class → get abilities → play the role.
character-class: get abilities → use them → discover what you're good at → the class finds you.

You don't decide who you are. You find out through experience. That's the whole point.

## License

MIT
