import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MatrixEditorComponent } from './matrix-editor.component';

describe('MatrixEditorComponent', () => {
  let component: MatrixEditorComponent;
  let fixture: ComponentFixture<MatrixEditorComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ MatrixEditorComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(MatrixEditorComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
