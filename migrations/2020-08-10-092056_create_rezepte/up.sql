CREATE TABLE IF NOT EXISTS `rezepte` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(70) COLLATE utf8_unicode_ci NOT NULL,
  `ingredients` text COLLATE utf8_unicode_ci NOT NULL,
  `preparation` text COLLATE utf8_unicode_ci NOT NULL,
  `experience` varchar(20) COLLATE utf8_unicode_ci DEFAULT NULL,
  `time_need` varchar(30) COLLATE utf8_unicode_ci DEFAULT NULL,
  `number_people` decimal(2,0) DEFAULT NULL,
  `created` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
  `owner` decimal(4,0) DEFAULT NULL,
  `rights` decimal(3,0) DEFAULT NULL,
  `category` int(11) DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `category` (`category`)
) ENGINE=MyISAM AUTO_INCREMENT=105 DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
