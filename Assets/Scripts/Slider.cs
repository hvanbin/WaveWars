using UnityEngine;
using System.Collections;
using UnityEngine.UI;

public class Slider : MonoBehaviour {
	public static int clockRate;
	// Use this for initialization
	void Start () {
		clockRate = 5;
	}
	
	// Update is called once per frame
	public void OnValueChanged(float newValue)
	{
		clockRate = (int)(newValue);
		this.GetComponentInChildren<Text>().text = "Turn Time = "+clockRate;
	}

}
