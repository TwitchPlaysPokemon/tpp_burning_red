_MomWakeUpText::
	text "MOM: Right."
	IF _GIRL
		line "All girls dream"
		cont "of traveling."
	ELSE
		line "All boys leave"
		cont "home some day."
	ENDC
	cont "It said so on TV."

	para "PROF.OAK, next"
	line "door, is looking"
	cont "for you."
	done

_MomHealText1::
	text "MOM: <PLAYER>!"
	line "You should take a"
	cont "quick rest."
	prompt

_MomHealText2::
	text "MOM: Oh good!"
	line "You and your"
	cont "#MON are"
	cont "looking great!"
	cont "Take care now!"
	done

_StandByMeText::
	text "There's a movie"
	IF _GIRL
		line "on TV. A girl"
		cont "with her hair in"
		cont "pigtails is walk-"
		cont "ing up a brick"
		cont "road."
	ELSE
		line "on TV. Four boys"
		cont "are walking on"
		cont "railroad tracks."
	ENDC

	para "I better go too."
	done

_TVWrongSideText::
	text "Oops, wrong side."
	done
