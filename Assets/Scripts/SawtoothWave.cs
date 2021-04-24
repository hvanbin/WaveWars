using UnityEngine;
using System.Collections;
using System;

public class SawtoothWave : Wave
{
    private bool counter;

    public override void Start()
    {
        base.Start();
        counter = false;
    }
    public override void Move()
	{
		if (!counter){
			shiftX (2);
		}
        shiftY(counter);

        counter = !counter;
    }
}
