<script setup lang="ts">
/**
 * Liste des supports (chaînes et stations) que je détiens, et leur gestion, 
 * grille de programmation, demandes reçues, équipe (US5, US6).
 *
 * Extrait de la page `/mon-compte/mes-supports` pour être également monté en
 * onglet de `/mon-compte/profil`, où l'utilisateur cherche naturellement ce
 * qu'il gère. Les deux points d'entrée partagent donc le même code.
 *
 * On devient détenteur de deux façons seulement : par la validation d'une
 * proposition de chaîne ou de station (on en est alors propriétaire), ou par
 * l'acceptation d'une invitation (co-détenteur ou programmateur).
 */
import {
  useMediaDetention,
  LIBELLES_ROLE_DETENTEUR,
  DESCRIPTIONS_ROLE_DETENTEUR,
  type DetenteurAPI,
} from '~/composables/useMediaDetention'
import type { EmissionAPI } from '~/composables/useMediaEmissions'
import { CADENCES_ORDONNEES, LIBELLES_CADENCE } from '~/composables/useMediaEmissions'
import { LIBELLES_NIVEAU_ALERTE, type AlerteCadence } from '~/composables/useMediaProgrammation'
import { porteurProgramme } from '~/composables/useMediaEquipe'

const { mesSupports, chargement, erreur } = useMediaDetention()
const { listerEmissionsDetenteur, creerEmission } = useMediaEmissions()
const { mesAlertesCadence } = useMediaProgrammation()
const {
  listerReferentielsEdition, obtenirThematiques, definirThematiques,
  obtenirCouverture, definirCouverture, erreur: erreurFiche,
} = useMediaSupport()

const supports = ref<DetenteurAPI[]>([])

/** Alertes de cadence, tous supports détenus confondus (FR-024). */
const alertes = ref<AlerteCadence[]>([])

/** Identifiant du support dont le panneau de gestion est déplié (un seul à la fois). */
const gestionOuverte = ref<string | null>(null)

/**
 * Programmes de chaque support, pour la gestion des épisodes ET le sélecteur
 * de la grille.
 *
 * Chargés à l'ouverture du panneau et mémorisés : sans eux le sélecteur reste
 * vide et aucun créneau n'est créable, la grille n'a alors rien à programmer.
 * Ce sont **tous** les programmes du détenteur, brouillons compris : ne montrer
 * que les publiés lui cacherait ce qu'il vient de créer.
 * Clé : l'identifiant du support.
 */
const emissionsParSupport = ref<Record<string, EmissionAPI[]>>({})

/** Thématiques et couverture, chargées à l'ouverture du panneau. */
const ficheParSupport = ref<Record<string, {
  thematiques: string[]
  continentale: boolean
  /** Support thématique (09v) : une thématique, aucun territoire à saisir.
   *  Décidé à la création ou en back-office, jamais ici — d'où la lecture
   *  seule : le détenteur constate la nature de son support, il ne la change
   *  pas. */
  estThematique: boolean
  territoires: string[]
}>>({})

const thematiquesRef = ref<{ id: string, nom: string }[]>([])
const territoiresRef = ref<{ id: string, nom: string }[]>([])

const chargerEmissions = async (detenteur: DetenteurAPI, force = false) => {
  if (!force && emissionsParSupport.value[detenteur.support_id]) return
  emissionsParSupport.value[detenteur.support_id] = await listerEmissionsDetenteur(
    detenteur.type_support,
    detenteur.support_id,
  )
}

const chargerFiche = async (detenteur: DetenteurAPI) => {
  if (ficheParSupport.value[detenteur.support_id]) return
  const [themes, couverture] = await Promise.all([
    obtenirThematiques(detenteur.type_support, detenteur.support_id),
    obtenirCouverture(detenteur.type_support, detenteur.support_id),
  ])
  ficheParSupport.value[detenteur.support_id] = {
    thematiques: themes.map(t => t.id),
    continentale: couverture?.couverture_continentale ?? false,
    estThematique: couverture?.est_thematique === true,
    territoires: couverture?.territoires.map(t => t.id) ?? [],
  }
}

/** Le sélecteur de la grille ne montre que ce que le serveur acceptera. */
const emissionsProgrammables = (supportId: string) =>
  (emissionsParSupport.value[supportId] ?? []).map(e => ({
    id: e.id,
    titre: e.titre,
    cadence: e.cadence,
    nombre_episodes: e.nombre_episodes,
  }))

const charger = async () => {
  supports.value = await mesSupports()
  alertes.value = await mesAlertesCadence()
  ouvrirSupportDemande()
}

/**
 * Déplie d'emblée le panneau du support désigné par `?support=<id>`.
 *
 * C'est ce que visent les passerelles venues des vitrines publiques (« Gérer ma
 * chaîne » dans une section de `/medias/tele`) : sans cela le détenteur
 * atterrit sur une liste refermée et doit y retrouver le support qu'il vient de
 * quitter. Un identifiant inconnu : support retiré entre-temps, lien recopié, 
 * est simplement ignoré.
 */
const route = useRoute()

const ouvrirSupportDemande = () => {
  const demande = route.query.support
  const supportId = Array.isArray(demande) ? demande[0] : demande
  if (!supportId) return
  const cible = supports.value.find(s => s.support_id === supportId)
  if (!cible) return
  gestionOuverte.value = cible.id
  ouvrirPanneau(cible)
}

onMounted(async () => {
  await charger()
  const referentiels = await listerReferentielsEdition()
  thematiquesRef.value = referentiels.thematiques
  territoiresRef.value = referentiels.territoires
})

const ouvrirPanneau = (detenteur: DetenteurAPI) => {
  chargerEmissions(detenteur)
  chargerFiche(detenteur)
}

const basculerGestion = (detenteur: DetenteurAPI) => {
  const id = detenteur.id
  gestionOuverte.value = gestionOuverte.value === id ? null : id
  if (gestionOuverte.value === id) ouvrirPanneau(detenteur)
}

// ── Création d'un programme ───────────────────────────────────
const creationPour = ref<DetenteurAPI | null>(null)
const creationErreur = ref<string | null>(null)
const nouveauProgramme = reactive({ titre: '', cadence: 'ponctuelle', description: '' })

const ouvrirCreationProgramme = (detenteur: DetenteurAPI) => {
  creationPour.value = detenteur
  creationErreur.value = null
  nouveauProgramme.titre = ''
  nouveauProgramme.cadence = 'ponctuelle'
  nouveauProgramme.description = ''
}

/**
 * Un programme naît **sans média et sans épisode** : c'est ce qui le distingue
 * de l'ancien « programme », qui était le contenu lui-même. Exiger un fichier
 * ici rendrait impossible de déclarer une série avant d'en avoir tourné le
 * premier épisode (FR-003).
 */
const creerProgramme = async () => {
  if (!creationPour.value) return
  if (!nouveauProgramme.titre.trim()) { creationErreur.value = 'Le titre est obligatoire.'; return }
  const detenteur = creationPour.value
  const cree = await creerEmission(detenteur.type_support, detenteur.support_id, {
    titre: nouveauProgramme.titre.trim(),
    cadence: nouveauProgramme.cadence,
    description: nouveauProgramme.description.trim(),
  })
  if (!cree) { creationErreur.value = 'Création impossible.'; return }
  creationPour.value = null
  await chargerEmissions(detenteur, true)
}

/** Programme dont les épisodes sont dépliés (un seul à la fois). */
const episodesOuverts = ref<string | null>(null)

const basculerEpisodes = (emissionId: string) => {
  episodesOuverts.value = episodesOuverts.value === emissionId ? null : emissionId
}

// ── Fiche du support : enregistrement ─────────────────────────
const enregistrementFiche = ref<string | null>(null)
const ficheEnregistree = ref<string | null>(null)

const enregistrerFiche = async (detenteur: DetenteurAPI) => {
  const fiche = ficheParSupport.value[detenteur.support_id]
  if (!fiche) return
  enregistrementFiche.value = detenteur.support_id
  ficheEnregistree.value = null
  const okThemes = await definirThematiques(detenteur.type_support, detenteur.support_id, fiche.thematiques)
  // Un support thématique n'a pas de couverture à écrire : le serveur refuse
  // l'appel (400), et pour cause — elle est déjà « tous les territoires ».
  const okCouverture = fiche.estThematique
    ? true
    : await definirCouverture(
      detenteur.type_support, detenteur.support_id, fiche.continentale, fiche.territoires,
    )
  enregistrementFiche.value = null
  if (okThemes && okCouverture) {
    ficheEnregistree.value = detenteur.support_id
    setTimeout(() => { ficheEnregistree.value = null }, 3000)
  }
}

const alertesDuSupport = (supportId: string) =>
  alertes.value.filter(a => a.support.id === supportId)

const dateCourte = (iso: string | null) =>
  iso ? new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'short' }).format(new Date(iso)) : '-' 

const LIBELLES_TYPE_SUPPORT: Record<DetenteurAPI['type_support'], string> = {
  chaine_tv: 'Chaîne de télévision',
  station_radio: 'Station de radio',
}

const iconeType = (type: DetenteurAPI['type_support']): [string, string] =>
  type === 'chaine_tv' ? ['fas', 'tv'] : ['fas', 'radio']

/** La page publique n'existe que si le support a un slug (fiche publiée). */
const lienPublic = (detenteur: DetenteurAPI): string | null => {
  if (!detenteur.support_slug) return null
  return detenteur.type_support === 'chaine_tv'
    ? `/medias/chaines/${detenteur.support_slug}`
    : `/medias/stations/${detenteur.support_slug}`
}

const dateFormatee = (iso: string) =>
  new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'long', year: 'numeric' })
    .format(new Date(iso))
</script>

<template>
  <div>
    <div v-if="erreur" class="mb-6 rounded-lg bg-af-live/5 border border-af-live/30 px-4 py-3 text-sm text-af-live">
      {{ erreur }}
    </div>

    <div v-if="chargement" class="flex justify-center py-16">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-af-chocolat"></div>
    </div>

    <div v-else-if="supports.length === 0" class="text-center py-16">
      <font-awesome-icon :icon="['fas', 'tv']" class="w-12 h-12 text-af-corps mb-4" />
      <p class="text-af-atone-2 mb-1">Vous ne détenez aucun support pour l’instant.</p>
      <p class="text-sm text-af-corps max-w-md mx-auto">
        On devient détenteur en faisant valider une proposition de chaîne ou de
        station : vous en êtes alors le propriétaire, ou en acceptant
        l’invitation d’un propriétaire.
      </p>
      <div class="mt-5 flex flex-wrap items-center justify-center gap-4 text-sm">
        <NuxtLink
          to="/mon-compte/propositions-medias"
          class="rounded-full bg-af-chocolat px-5 py-2 font-medium text-af-encre transition-colors hover:bg-af-chocolat/90"
        >
          Mes propositions de médias
        </NuxtLink>
        <NuxtLink
          to="/mon-compte/invitations-medias"
          class="rounded-full border border-af-bordure bg-white px-5 py-2 font-medium text-af-corps transition-colors hover:bg-af-fond"
        >
          Mes invitations
        </NuxtLink>
      </div>
    </div>

    <!-- Alertes de cadence, tous supports confondus : une échéance dépassée
         doit se voir avant d'ouvrir un panneau (FR-024). -->
    <div v-if="!chargement && alertes.length" class="mb-6 space-y-2">
      <h2 class="font-oswald text-lg font-bold text-af-encre">
        Échéances de vos programmes
      </h2>
      <ul class="space-y-2">
        <li
          v-for="alerte in alertes"
          :key="alerte.emission.id"
          class="rounded-lg border px-4 py-3 text-sm"
          :class="alerte.niveau === 'depassee'
            ? 'border-af-live/30 bg-af-live/5 text-af-live'
            : 'border-af-chocolat/20 bg-af-chocolat/5 text-af-chocolat'"
        >
          <div class="flex flex-wrap items-baseline justify-between gap-2">
            <span class="font-semibold">{{ alerte.emission.titre }}</span>
            <span class="text-xs uppercase tracking-wide">
              {{ LIBELLES_NIVEAU_ALERTE[alerte.niveau] }}
            </span>
          </div>
          <p class="text-xs mt-1 opacity-80">
            {{ alerte.support.nom }} · cadence {{ LIBELLES_CADENCE[alerte.cadence] || alerte.cadence }}
            <template v-if="alerte.prochaine_echeance">
              · attendu le {{ dateCourte(alerte.prochaine_echeance) }}
            </template>
          </p>
          <!-- Le détenteur a fait sa part : c'est la file de modération qui n'a
               pas suivi. L'alerte ne doit pas l'accuser. -->
          <p v-if="alerte.episodes_en_attente" class="text-xs mt-1">
            {{ alerte.episodes_en_attente }} épisode(s) déjà soumis, en attente de validation.
          </p>
          <p v-else-if="alerte.niveau === 'aucun_episode'" class="text-xs mt-1">
            Ce programme n'a aucun épisode publié : ses créneaux n'annoncent rien.
          </p>
        </li>
      </ul>
    </div>

    <!-- `v-if` autonome et non `v-else` : le bandeau d'alertes s'intercale
         au-dessus et romprait la chaîne conditionnelle. -->
    <ul v-if="!chargement && supports.length" class="space-y-4">
      <li
        v-for="detenteur in supports"
        :key="detenteur.id"
        class="bg-white rounded-xl shadow-sm border border-af-bordure overflow-hidden"
      >
        <div class="p-5">
          <div class="flex items-start gap-4">
            <!-- Visuel du support, ou repli iconographique selon le type. -->
            <div class="shrink-0 h-16 w-16 rounded-lg bg-af-fond overflow-hidden flex items-center justify-center">
              <img
                v-if="detenteur.support_image"
                :src="detenteur.support_image"
                :alt="detenteur.support_nom || 'Support'"
                class="h-full w-full object-cover"
              >
              <font-awesome-icon v-else :icon="iconeType(detenteur.type_support)" class="w-6 h-6 text-af-corps" />
            </div>

            <div class="min-w-0 flex-1">
              <h2 class="font-semibold text-af-encre truncate">
                {{ detenteur.support_nom || 'Support sans nom' }}
              </h2>
              <div class="mt-1.5 flex flex-wrap items-center gap-2">
                <span class="rounded-full bg-af-fond px-3 py-1 text-xs font-medium text-af-corps">
                  {{ LIBELLES_TYPE_SUPPORT[detenteur.type_support] }}
                </span>
                <span
                  class="rounded-full bg-af-chocolat/10 px-3 py-1 text-xs font-bold uppercase tracking-wide text-af-chocolat"
                  :title="DESCRIPTIONS_ROLE_DETENTEUR[detenteur.role]"
                >
                  {{ LIBELLES_ROLE_DETENTEUR[detenteur.role] }}
                </span>
              </div>
              <p class="mt-2 text-xs text-af-corps">
                Détenteur depuis le {{ dateFormatee(detenteur.designe_at) }}
              </p>
            </div>
          </div>

          <div class="mt-4 flex flex-wrap items-center gap-3">
            <NuxtLink
              v-if="lienPublic(detenteur)"
              :to="lienPublic(detenteur)!"
              class="inline-flex items-center gap-2 rounded-full border border-af-bordure bg-white px-4 py-1.5 text-sm font-medium text-af-corps transition-colors hover:bg-af-fond"
            >
              <font-awesome-icon :icon="['fas', 'eye']" class="w-3.5 h-3.5" />
              Voir la page publique
            </NuxtLink>

            <button
              type="button"
              class="inline-flex cursor-pointer items-center gap-2 rounded-full px-4 py-1.5 text-sm font-medium transition-colors"
              :class="gestionOuverte === detenteur.id
                ? 'bg-gray-800 text-af-encre'
                : 'bg-af-chocolat text-white hover:bg-af-chocolat/90'"
              :aria-expanded="gestionOuverte === detenteur.id"
              @click="basculerGestion(detenteur)"
            >
              <font-awesome-icon :icon="['fas', 'sliders']" class="w-3.5 h-3.5" />
              {{ gestionOuverte === detenteur.id ? 'Fermer la gestion' : 'Gérer' }}
            </button>
          </div>
        </div>

        <!-- Panneau de gestion déplié en place : pas de navigation, on reste
             dans la liste de ses supports. -->
        <div v-if="gestionOuverte === detenteur.id" class="border-t border-af-bordure bg-af-fond/60 p-5 space-y-8">
          <!-- Programmes et épisodes : le cœur de ce que gère un détenteur -->
          <section>
            <div class="flex flex-wrap items-baseline justify-between gap-3 mb-3">
              <h3 class="font-oswald text-lg font-bold text-af-encre">Programmes</h3>
              <button
                type="button"
                class="rounded-full bg-af-chocolat px-4 py-1.5 text-sm font-medium text-af-encre transition-colors hover:bg-af-chocolat/90"
                @click="ouvrirCreationProgramme(detenteur)"
              >
                + Nouveau programme
              </button>
            </div>

            <p
              v-if="(emissionsParSupport[detenteur.support_id] ?? []).length === 0"
              class="text-sm text-af-atone"
            >
              Aucun programme pour l'instant. Un programme se déclare sans fichier :
              ses épisodes viendront ensuite.
            </p>

            <ul v-else class="space-y-3">
              <li
                v-for="emission in emissionsParSupport[detenteur.support_id]"
                :key="emission.id"
                class="rounded-lg border border-af-bordure bg-white"
              >
                <div class="flex flex-wrap items-center justify-between gap-3 p-4">
                  <div class="min-w-0">
                    <p class="font-semibold text-af-encre truncate">{{ emission.titre }}</p>
                    <p class="text-xs text-af-atone mt-0.5">
                      {{ emission.nombre_episodes }} épisode(s) publié(s)
                      · {{ LIBELLES_CADENCE[emission.cadence] || emission.cadence }}
                      <span v-if="emission.episodes_en_attente" class="text-af-chocolat">
                        · {{ emission.episodes_en_attente }} en attente
                      </span>
                      <span v-if="emission.episodes_rejetes" class="text-af-live">
                        · {{ emission.episodes_rejetes }} refusé(s)
                      </span>
                    </p>
                  </div>
                  <button
                    type="button"
                    class="rounded-full border border-af-bordure px-4 py-1.5 text-sm font-medium text-af-corps transition-colors hover:bg-af-fond"
                    @click="basculerEpisodes(emission.id)"
                  >
                    {{ episodesOuverts === emission.id ? 'Masquer les épisodes' : 'Gérer les épisodes' }}
                  </button>
                </div>

                <div v-if="episodesOuverts === emission.id" class="border-t border-af-bordure p-4 space-y-6">
                  <MediaGestionEpisodes
                    :emission-id="emission.id"
                    :emission-titre="emission.titre"
                    :type-support="detenteur.type_support"
                    :sombre="false"
                    @change="chargerEmissions(detenteur, true)"
                  />

                  <!-- L'équipe DU PROGRAMME (010, FR-011) : distincte de celle
                       de son support, elles coexistent sans recopie. Le
                       discriminant se déduit de la famille du support. -->
                  <div class="border-t border-af-bordure pt-4">
                    <MediaGestionEquipe
                      :type-porteur="porteurProgramme(detenteur.type_support)"
                      :porteur-id="emission.id"
                      base="membre"
                      :titre="`Équipe de « ${emission.titre} »`"
                    />
                  </div>
                </div>
              </li>
            </ul>
          </section>

          <section>
            <h3 class="font-oswald text-lg font-bold text-af-encre mb-3">Grille de programmation</h3>
            <!-- Les programmes du support, chargés à l'ouverture du panneau :
                 c'est la seule source du sélecteur, sans laquelle aucun créneau
                 n'est créable. Le créneau vise un PROGRAMME ; l'épisode diffusé
                 se déduit de la date d'effet. -->
            <MediaGrilleProgrammation
              :type-support="detenteur.type_support"
              :support-id="detenteur.support_id"
              :emissions="emissionsProgrammables(detenteur.support_id)"
              :modifiable="true"
            />
            <p
              v-if="emissionsProgrammables(detenteur.support_id).length === 0"
              class="mt-2 text-sm text-af-atone"
            >
              Ce support n'a encore aucun programme : créez-en un avant de bâtir sa grille.
            </p>
          </section>

          <!-- Thématiques et couverture : ce qui rend le support trouvable -->
          <section v-if="ficheParSupport[detenteur.support_id]">
            <h3 class="font-oswald text-lg font-bold text-af-encre mb-3">
              {{ ficheParSupport[detenteur.support_id]!.estThematique
                ? 'Thématique du support'
                : 'Thématiques &amp; couverture' }}
            </h3>
            <div class="space-y-5 rounded-lg border border-af-bordure bg-white p-4">
              <MediaSelecteurThematiques
                :model-value="ficheParSupport[detenteur.support_id]!.thematiques"
                :options="thematiquesRef"
                :sombre="false"
                :unique="ficheParSupport[detenteur.support_id]!.estThematique"
                @update:model-value="ficheParSupport[detenteur.support_id]!.thematiques = $event"
              />
              <MediaSelecteurCouverture
                v-if="!ficheParSupport[detenteur.support_id]!.estThematique"
                :continentale="ficheParSupport[detenteur.support_id]!.continentale"
                :territoires="ficheParSupport[detenteur.support_id]!.territoires"
                :options="territoiresRef"
                :sombre="false"
                @update:continentale="ficheParSupport[detenteur.support_id]!.continentale = $event"
                @update:territoires="ficheParSupport[detenteur.support_id]!.territoires = $event"
              />
              <p v-else class="text-sm text-af-atone">
                Support thématique : il couvre d'office tous les territoires,
                il n'y a pas de couverture à saisir.
              </p>

              <p v-if="erreurFiche" class="text-sm text-af-live">{{ erreurFiche }}</p>
              <p v-if="ficheEnregistree === detenteur.support_id" class="text-sm text-af-vert">
                Fiche enregistrée.
              </p>

              <div class="flex justify-end">
                <button
                  type="button"
                  :disabled="enregistrementFiche === detenteur.support_id"
                  class="rounded-full bg-af-chocolat px-5 py-2 text-sm font-medium text-af-encre transition-colors hover:bg-af-chocolat/90 disabled:opacity-50"
                  @click="enregistrerFiche(detenteur)"
                >
                  Enregistrer la fiche
                </button>
              </div>
            </div>
          </section>

          <section>
            <h3 class="font-oswald text-lg font-bold text-af-encre mb-3">
              Demandes reçues
            </h3>
            <p class="mb-3 text-sm text-af-atone-2">
              Les idées de contenu déposées par les visiteurs et les demandes
              d'animation adressées à ce support.
            </p>
            <MediaDemandesEngagementSupport
              :type-support="detenteur.type_support"
              :support-id="detenteur.support_id"
              :mon-role="detenteur.role"
            />
          </section>

          <!-- Équipe ÉDITORIALE du support (010) : les personnes qui font la
               chaîne ou la station, publiées sur ses pages. À ne pas confondre
               avec « Gestion des accès » ci-dessous, qui distribue des DROITS. -->
          <section>
            <MediaGestionEquipe
              :type-porteur="detenteur.type_support"
              :porteur-id="detenteur.support_id"
              base="membre"
              titre="Équipe éditoriale"
            />
          </section>

          <!-- Anciennement « Équipe du support » : renommé par la feature 010 :
               ce panneau ne décrit personne, il ouvre des accès. Deux blocs
               « équipe » sans rapport dans la même page étaient une invitation
               à se tromper de formulaire. -->
          <section>
            <h3 class="font-oswald text-lg font-bold text-af-encre mb-3">Gestion des accès</h3>
            <p class="mb-3 text-sm text-af-atone-2">
              Qui peut administrer ce support : co-détenteurs et programmateurs.
              Sans effet sur l'équipe éditoriale publiée ci-dessus.
            </p>
            <MediaGestionCoDetenteurs
              :type-support="detenteur.type_support"
              :support-id="detenteur.support_id"
              :mon-role="detenteur.role"
            />
          </section>
        </div>
      </li>
    </ul>

    <!-- Création d'un programme : sans média, volontairement (FR-003) -->
    <div
      v-if="creationPour"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4"
      @click.self="creationPour = null"
    >
      <div class="w-full max-w-md rounded-xl bg-white p-6">
        <h3 class="font-oswald text-xl font-bold text-af-encre mb-1">Nouveau programme</h3>
        <p class="text-sm text-af-atone mb-4">
          Sur {{ creationPour.support_nom }}. Le programme est créé en brouillon ;
          il devient public dès qu'un de ses épisodes est validé.
        </p>

        <p v-if="creationErreur" class="mb-4 rounded-lg bg-af-live/5 border border-af-live/30 px-4 py-2 text-sm text-af-live">
          {{ creationErreur }}
        </p>

        <div class="space-y-3">
          <div>
            <label class="block text-sm text-af-corps mb-1">Titre *</label>
            <input
              v-model="nouveauProgramme.titre"
              type="text"
              class="w-full rounded-lg border border-af-bordure px-3 py-2 text-sm outline-none focus:border-af-encre"
              placeholder="Ex : Le Grand Débat"
            >
          </div>
          <div>
            <label class="block text-sm text-af-corps mb-1">Périodicité</label>
            <!-- Les quatre valeurs et leurs libellés viennent de la table
                 partagée : le public, l'espace membre et le back-office lisent
                 le même mot pour la même valeur (010, FR-041). -->
            <select
              v-model="nouveauProgramme.cadence"
              class="w-full rounded-lg border border-af-bordure px-3 py-2 text-sm outline-none focus:border-af-encre"
            >
              <option v-for="c in CADENCES_ORDONNEES" :key="c" :value="c">
                {{ LIBELLES_CADENCE[c] }}
              </option>
            </select>
            <p class="text-xs text-af-atone mt-1">
              Elle sert à vous alerter d'une échéance sans épisode, pas à décider de la diffusion.
            </p>
          </div>
          <div>
            <label class="block text-sm text-af-corps mb-1">Description</label>
            <textarea
              v-model="nouveauProgramme.description"
              rows="3"
              class="w-full rounded-lg border border-af-bordure px-3 py-2 text-sm outline-none focus:border-af-encre"
            />
          </div>
        </div>

        <div class="mt-6 flex justify-end gap-3">
          <button type="button" class="text-sm text-af-atone hover:text-af-encre" @click="creationPour = null">
            Annuler
          </button>
          <button
            type="button"
            class="rounded-full bg-af-chocolat px-5 py-2 text-sm font-medium text-af-encre hover:bg-af-chocolat/90"
            @click="creerProgramme"
          >
            Créer le programme
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
