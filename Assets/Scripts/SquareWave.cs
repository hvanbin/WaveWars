using UnityEngine;
using System.Collections;
using System;

public class SquareWave : Wave
{
    private int counter;

    public override void Move()
    {
        if (counter == 0) shiftY(false);
        else if (counter == 2) shiftY(true);
        else shiftX();

        counter = (counter == 3) ? 0 : counter + 1;
    }

    public override void Start()
    {
        base.Start();
        counter = 0;
	}
}
