import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RoundChartComponent } from './round-chart.component';

describe('RoundChartComponent', () => {
  let component: RoundChartComponent;
  let fixture: ComponentFixture<RoundChartComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ RoundChartComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(RoundChartComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
