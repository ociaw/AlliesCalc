import { Component, OnInit, ViewChild } from '@angular/core';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';
import { CumulativeStats, RoundStats, RoundSummary } from 'allies-calc-rs';
import { CalcService } from './calc.service';
import { Unit } from './model/unit';
import { SetupForceComponent } from './setup-force/setup-force.component';
import { RoundChartComponent } from './round-chart/round-chart.component';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
})
export class AppComponent implements OnInit {
  title = 'angular-app';
  attackerTitle = "Attacker";
  defenderTitle = "Defender";
  roundSummaries: Observable<RoundSummary[]>;
  lastRoundSummary: Observable<RoundSummary>;
  roundStats: Observable<RoundStats>;
  cumulativeStats: Observable<CumulativeStats>;
  availableUnits: Unit[] = [];

  @ViewChild("attackerSetup") attackers!: SetupForceComponent;
  @ViewChild("defenderSetup") defenders!: SetupForceComponent;
  @ViewChild("roundChart") roundChart!: RoundChartComponent;

  constructor(private calc: CalcService) {
    this.roundSummaries = calc.roundSummaries();
    this.lastRoundSummary = this.roundSummaries.pipe(map(summaries => summaries[summaries.length]));
    this.roundStats = calc.roundStats();
    this.cumulativeStats = calc.cumulativeStats();
    calc.availableUnits().subscribe({
      next: array => this.availableUnits = array
    });
    calc.roundSummaries().subscribe({
      next: summaries => this.roundChart.summaries = summaries
    });
  }

  nextRound(_event?: Event) {
    this.calc.advanceRound();
  }

  setup(_event?: Event) {
    this.calc.reset(this.attackers.selectedUnits, this.defenders.selectedUnits);
  }

  ngOnInit(): void {
    import("allies-calc-rs").then((wasm) => {
      wasm.setPanicHook();
    });
  }
}
