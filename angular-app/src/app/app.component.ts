import { Component } from '@angular/core';
import { CalcService } from './calc.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title = 'angular-app';
  roundIndex = 0;
  attackerWinP = 0.0;
  defenderWinP = 0.0;
  drawP = 0.0;

  constructor(private calc: CalcService) {}
}
