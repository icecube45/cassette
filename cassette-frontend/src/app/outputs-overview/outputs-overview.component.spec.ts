import { ComponentFixture, TestBed } from '@angular/core/testing';

import { OutputsOverviewComponent } from './outputs-overview.component';

describe('OutputsOverviewComponent', () => {
  let component: OutputsOverviewComponent;
  let fixture: ComponentFixture<OutputsOverviewComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ OutputsOverviewComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(OutputsOverviewComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
