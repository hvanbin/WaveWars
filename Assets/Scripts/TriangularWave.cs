using UnityEngine;
using System.Collections;
using System;

public class TriangularWave : Wave
{
    private bool counter;

    public override void Start()
    {
        base.Start();
        counter = false;
    }
    public override void Move()
    {
        shiftX();
        shiftY(counter);
        counter = !counter;
    }
}
