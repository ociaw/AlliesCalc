# Axis and Allies Calculator

[AABattleground.com](https://aabattleground.com/)

The most accurate and precise odds calculator for battles in the game *Axis and Allies*.

Most odd "calculators" are actually simulators - they simulate thousands upon thousands of
different random outcomes and aggregating the results. This can be very fast, but is often
inaccurate and the result can often vary wildly from run to run, with the only solution being to
crank up the number of simulations.

Unlike simulators, however, this is a true calculator. This calculator examines every possible
outcome, discarding the most unlikely outcomes and combining the rest to give you the most accurate
result as quickly as possible.

## Variants

Currently only `Axis and Allies 1942 2nd Edition` is supported; however, the underlying calculation
engine is designed to be very flexible and will offer a diverse number of rulesets in the future.
As another future feature, rulesets will be able to support modified/custom units; for example, a
bombarding submarine or a fighter that always hits.

## Device Support

A modern browser with WebAssembly support is required, such as Firefox, Google Chrome, or Microsoft
Edge (Internet Explorer is not supported). The calculator runs entirely within the browser - once
you've loaded the page, an internet connection is unnecessary. As such, calculation speed depends
entirely on your device - very large battles may only be feasible on full desktop or laptop
computers.

# Nitty Gritty Details (How it's built)

The calculation engine is written entirely in Rust.

This is compiled to WebAssembly and packaged with WasmPack. The resulting ES module is then pulled
in and used in the Angular app.

The calculator works round by round, taking each pending combat from the last round and enumerating
and resolving every possible outcome. For each combat, it first selects the rolls for the attacking
and defending forces. It then expands those rolls into every possible hit combination. The hit
combinations are then used to select survivors from the receiving force, and aggregated into a list
for both attackers and defenders. Each surviving attacking force is paired with each surviving
defending force, which are then combined with the results of each other combat. In the end we get a
`RoundResult` and the whole thing starts over. Whew. (A diagram would probably helpful here.)

Very unlikely (configurable threshold) surviving attacker-defender pairs are "pruned" before they
can be added to the final round result. This ensures that we don't waste memory tracking frivolous
outcomes.

## Rulesets

Rulesets are built by implenting 5 different traits: `Unit`, `Hit`, `BattlePhase`, `RollSelector`,
and `SurvivorSelector`.

### Unit
A unit is a combatant on the battlefield, such as a *submarine*, *fighter*, or *tank*.

### Battle Phase
A battle is composed of different phases occuring in sequence. Each ruleset defines their own
phases to use. The `1942 2nd Edition` ruleset defines the following phases:

- PreBattle - Each ruleset must define a prebattle phase, which represents the "calm before the storm".
- Bombardment - Bombarding units get to bombard here.
- AntiAir - Anti-Air units fire here.
- SurpriseStrike - Submarines can fire here if there's not a destroyer present.
- General - Everyone else fires.

### Hit
Not all units can hit every other unit. For example, Anti Air guns can only hit airplanes, while
submarines can't hit them at all. This concept is represent by a `Hit`.

### Roll Selector
A more apt name might be "Dice Selector". `Roll selectors` choose the rolls that are to be made for
a given force of units. For example, given an attacking force of 1 submarine and 1 bomber, the roll
selector will return 1 `Not Submarine` hit with a strength of 4 and 1 `Not Aircraft` hit with a
strength of 2.

### Survivor Selector
The job of the surivor selector is to take a quantity of `Hit`s and apply them to a force of units
by selecting the surviors appropriately. For example, a `Not Submarine` hit cannot be applied to a
submarine, and a `Not Aircraft` hit can't be applied to a fighter.
