import { Injectable } from '@angular/core';
import { Observable, BehaviorSubject, Subject } from 'rxjs'
import { RoundManager, Statistics } from "allies-calc-rs";

@Injectable({
  providedIn: 'root'
})
export class CalcService {
  private roundManager!: RoundManager;
  private inited = new BehaviorSubject<boolean>(false);
  private statsSubject = new Subject<Statistics>();
  private countSubject = new Subject<number>();
  constructor() {
    import("allies-calc-rs").then((wasm) => {
      wasm.set_panic_hook();
      this.roundManager = wasm.RoundManager.new();
      this.inited.next(true);
    });
  }

  /**
   * advanceRound
*/
  public advanceRound() {
    if (!this.inited.getValue()) {
      return;
    }
    this.statsSubject.next(this.roundManager.stats());
    this.countSubject.next(this.roundManager.round_index() + 1);
    this.roundManager.advance_round();
  }

  /**
   * stats
  */
  public stats(): Observable<Statistics> {
    return this.statsSubject.asObservable();
  }

  public roundCount(): Observable<number> {
    return this.countSubject.asObservable();
  }
}
