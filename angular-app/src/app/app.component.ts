import { Component, OnInit, ViewChild } from '@angular/core';
import { Observable } from 'rxjs';
import { CumulativeStats, RoundStats } from 'allies-calc-rs';
import { CalcService } from './calc.service';
import { Unit } from './model/unit';
import { SetupForceComponent } from './setup-force/setup-force.component';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {
  title = 'angular-app';
  attackerTitle = "Attacker";
  defenderTitle = "Defender";
  roundStats: Observable<RoundStats>;
  cumulativeStats: Observable<CumulativeStats>;
  availableUnits: Unit[] = [];

  @ViewChild("attackerSetup") attackers!: SetupForceComponent;
  @ViewChild("defenderSetup") defenders!: SetupForceComponent;

  constructor(private calc: CalcService) {
    this.roundStats = calc.roundStats();
    this.cumulativeStats = calc.cumulativeStats();
    calc.availableUnits().subscribe({
      next: array => this.availableUnits = array
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
