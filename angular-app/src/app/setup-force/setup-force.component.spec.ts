import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SetupForceComponent } from './setup-force.component';

describe('SetupForceComponent', () => {
  let component: SetupForceComponent;
  let fixture: ComponentFixture<SetupForceComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ SetupForceComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(SetupForceComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
