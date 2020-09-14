import { Component } from '@angular/core';
import { CalcService } from './calc.service';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators'

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title = 'angular-app';
  roundNumber: Observable<number>;
  attackerWinP: Observable<number>;
  defenderWinP: Observable<number>;
  drawP: Observable<number>;

  constructor(private calc: CalcService) {
    this.roundNumber = calc.roundCount();
    this.attackerWinP = calc.stats().pipe(
      map(s => s.attacker_win_p())
    );
    this.defenderWinP = calc.stats().pipe(
      map(s => s.defender_win_p())
    );
    this.drawP = calc.stats().pipe(
      map(s => s.draw_p())
    );
  }

  nextRound(event?: Event) {
    this.calc.advanceRound();
  }
}
