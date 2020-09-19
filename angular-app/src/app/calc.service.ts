import { Injectable } from '@angular/core';
import { Observable, Subject } from 'rxjs'
import { Battle, CumulativeStats, RoundStats, Ruleset } from "allies-calc-rs";
import { Unit } from './model/unit';

@Injectable({
  providedIn: 'root'
})
export class CalcService {
  private inited = false;
  private battle!: Battle;
  private roundStatsSubject = new Subject<RoundStats>();
  private cumulativeStatsSubject = new Subject<CumulativeStats>();
  private availableUnitsSubject = new Subject<Unit[]>();
  constructor() {
    import("allies-calc-rs").then((wasm) => {
      this.inited = true;

      let unitProvider = new wasm.UnitProvider(wasm.Ruleset.AA1942_2E);
      let units: Unit[] = [];
      let unitCount = unitProvider.getUnitCount();
      for (let i = 0; i < unitCount; i++) {
        let name = unitProvider.getUnitName(i);
        let ipc = unitProvider.getUnitIpc(i);
        let attack = unitProvider.getUnitAttack(i);
        let defense = unitProvider.getUnitDefense(i);
        units.push(new Unit(i, name, ipc, attack, defense));
      }
      this.availableUnitsSubject.next(units);

      this.battle = wasm.Battle.default();
      this.roundStatsSubject.next(this.battle.roundStats());
      this.cumulativeStatsSubject.next(this.battle.cumulativeStats());
    });
  }

  public reset(attackers: [Unit, number][], defenders: [Unit, number][]) {
    import("allies-calc-rs").then((wasm) => {
      let builder = new wasm.BattleBuilder(wasm.Ruleset.AA1942_2E);
      for (let tuple of attackers) {
        builder.addAttacker(tuple[0].id, tuple[1])
      }
      for (let tuple of defenders) {
        builder.addDefender(tuple[0].id, tuple[1])
      }
      this.battle = builder.build();
      this.roundStatsSubject.next(this.battle.roundStats());
      this.cumulativeStatsSubject.next(this.battle.cumulativeStats());
    });
  }

  /**
   * advanceRound
  */
  public advanceRound() {
    if (!this.inited || this.battle.isComplete()) {
      return;
    }
    this.battle.advance_round();
    this.roundStatsSubject.next(this.battle.roundStats());
    this.cumulativeStatsSubject.next(this.battle.cumulativeStats());
  }

  /**
   * roundStats
   */
  public roundStats(): Observable<RoundStats> {
    return this.roundStatsSubject.asObservable();
  }

  /**
   * stats
  */
  public cumulativeStats(): Observable<CumulativeStats> {
    return this.cumulativeStatsSubject.asObservable();
  }

  /**
   * availableUnits
   */
  public availableUnits(): Observable<Unit[]> {
    return this.availableUnitsSubject.asObservable();
  }
}
