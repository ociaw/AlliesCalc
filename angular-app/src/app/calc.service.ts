import { Injectable } from '@angular/core';
import { RoundManager, Statistics } from "allies-calc-rs-wasm";

@Injectable({
  providedIn: 'root'
})
export class CalcService {
  private roundManager: RoundManager
  constructor() {
      this.roundManager = RoundManager.new();
  }

  /**
   * nextRound
  */
  public nextRound(): Statistics {
      this.roundManager.advance_round();
      return this.roundManager.stats();
  }

  public roundCount(): Number {
      return Number(this.roundManager.round_index().valueOf()) + 1;
  }
}
