CREATE TABLE IF NOT EXISTS `user` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `password` varchar(256) COLLATE utf8_unicode_ci NOT NULL,
  `username` varchar(256) COLLATE utf8_unicode_ci NOT NULL,
  `name` varchar(256) COLLATE utf8_unicode_ci DEFAULT NULL,
  `role` int(11) DEFAULT NULL,
  `created` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=8 DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
