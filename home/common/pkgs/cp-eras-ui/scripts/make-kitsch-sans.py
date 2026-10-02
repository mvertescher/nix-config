"""Derive CP Eras Kitsch Sans Bold from pinned GNU FreeFont 20120503.

Build-only tool. Requires FontTools; never edits the upstream file.
"""
from array import array
from pathlib import Path
import argparse
import hashlib
import json
import os
import tempfile
from fontTools.ttLib import TTFont
from fontTools.ttLib.tables._g_l_y_f import GlyphCoordinates
from fontTools.ttLib.tables.ttProgram import Program

SOURCE_SHA256 = '982534a3731416a15e2756601721f26053f68bf4239011550f3dd23ce6308215'
OUTPUT_SHA256 = '14d51cf24626c4c3103843d71d63b7a432cdc9945693397d193eeaf06827550c'
OLD_PROGRAM_SHA256 = '6bc1bd1f97b2d36628f8bfd184f466923c04c0f20c181989c205f139d48448ea'
NEW_PROGRAM_SHA256 = 'd79cce4e0db1a9c6a788a7eec558de2a3541ca3c4cc77aa5925290089227e6d6'
REMAP = ((8, 11, 12), (9, 12, 13), (25, 13, 15), (52, 14, 16),
         (68, 12, 13), (98, 11, 12), (124, 11, 12), (125, 12, 13))
BS15 = ((230, 568), (230, 0), (80, 0), (80, 729), (304, 729),
        (436, 440), (564, 729), (790, 729), (790, 0), (640, 0),
        (640, 568), (611, 440), (476, 220), (396, 220), (260, 440))
FAMILY = 'CP Eras Kitsch Sans'
FULL = 'CP Eras Kitsch Sans Bold'
POSTSCRIPT = 'CPErasKitschSansBold'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def changed_glyphs(original, derived):
    order = original.getGlyphOrder()
    assert order == derived.getGlyphOrder()
    before, after = original.getTableData('glyf'), derived.getTableData('glyf')
    offsets_a, offsets_b = original['loca'].locations, derived['loca'].locations
    return [name for i, name in enumerate(order)
            if before[offsets_a[i]:offsets_a[i + 1]] !=
               after[offsets_b[i]:offsets_b[i + 1]]]


def audit_record():
    return {'source_sha256': SOURCE_SHA256, 'output_sha256': OUTPUT_SHA256,
            'family': FAMILY, 'postscript': POSTSCRIPT,
            'changed_tables': ['glyf', 'head', 'loca', 'name'],
            'changed_glyphs': ['M'], 'other_glyphs_exact': 2910,
            'M_changed_point_indices_from_CG_control': [5, 11, 12, 13, 14],
            'M_instruction_bytes': 138,
            'M_instruction_operand_offsets': [x[0] for x in REMAP],
            'unchanged_shared_hint_tables': ['prep', 'fpgm', 'cvt '],
            'unchanged_metrics': True, 'embedded_license_preserved': True}


def atomic_audit(audit):
    fd, temp = tempfile.mkstemp(prefix=audit.name + '.', suffix='.tmp', dir=audit.parent)
    try:
        with os.fdopen(fd, 'w') as stream:
            json.dump(audit_record(), stream, indent=2)
            stream.write('\n')
        os.chmod(temp, 0o644)
        os.replace(temp, audit)
    finally:
        Path(temp).unlink(missing_ok=True)


def names(font):
    # Preserve every other name record, especially copyright, license,
    # license URL, vendor, style, credits and version. Make all localized
    # full-name records unambiguously distinct from FreeSans.
    for record in font['name'].names:
        value = {1: FAMILY, 3: 'GNU FreeFont 20120503; CP Eras Kitsch Sans Bold M-BS15',
                 4: FULL, 6: POSTSCRIPT}.get(record.nameID)
        if value is not None:
            record.string = value.encode('mac_roman' if record.platformID == 1 else 'utf_16_be')


def build(source, output, audit):
    assert source.resolve() != output.resolve()
    assert sha(source.read_bytes()) == SOURCE_SHA256, 'upstream FreeSans input changed'
    assert not output.is_symlink() and not audit.is_symlink()
    if output.is_file() and sha(output.read_bytes()) == OUTPUT_SHA256:
        # A prior interrupted hook may have written the font but not its
        # audit. Its pinned hash proves the complete font; repair the audit.
        if not audit.is_file() or audit.read_text() != json.dumps(audit_record(), indent=2) + '\n':
            atomic_audit(audit)
        return
    original = TTFont(source, recalcBBoxes=False, recalcTimestamp=False)
    font = TTFont(source, recalcBBoxes=False, recalcTimestamp=False)
    g = font['glyf']['M']
    old = list(map(tuple, g.coordinates))
    flags = list(g.flags)
    assert g.numberOfContours == 1 and g.endPtsOfContours == [12]
    assert len(old) == len(flags) == 13 and all(f & 1 for f in flags)
    assert old == [(230, 568), (230, 0), (80, 0), (80, 729), (304, 729),
                   (436, 149), (564, 729), (790, 729), (790, 0),
                   (640, 0), (640, 568), (511, 0), (361, 0)]
    old_program = g.program.getBytecode()
    assert len(old_program) == 138 and sha(old_program) == OLD_PROGRAM_SHA256
    program = bytearray(old_program)
    for offset, before, after in REMAP:
        assert program[offset] == before
        program[offset] = after
    assert sha(program) == NEW_PROGRAM_SHA256
    control = old[:11] + [old[10]] + old[11:] + [old[12]]
    assert [i for i, (a, b) in enumerate(zip(control, BS15)) if a != b] == [5, 11, 12, 13, 14]
    g.coordinates = GlyphCoordinates(BS15)
    g.flags = array('B', flags[:11] + [flags[10]] + flags[11:] + [flags[12]])
    g.endPtsOfContours = [14]
    remapped = Program()
    remapped.fromBytecode(bytes(program))
    g.program = remapped
    names(font)
    fd, temp = tempfile.mkstemp(prefix=output.name + '.', suffix='.tmp', dir=output.parent)
    os.close(fd)
    try:
        font.save(temp)
        derived = TTFont(temp, recalcBBoxes=False, recalcTimestamp=False)
        actual = derived['glyf']['M']
        assert list(map(tuple, actual.coordinates)) == list(BS15)
        assert list(actual.flags) == list(g.flags) and actual.endPtsOfContours == [14]
        assert actual.program.getBytecode() == bytes(program)
        assert derived['hmtx'].metrics['M'] == original['hmtx'].metrics['M'] == (870, 80)
        assert derived['OS/2'].usWeightClass == original['OS/2'].usWeightClass == 600
        assert derived.getGlyphOrder() == original.getGlyphOrder()
        assert (derived['head'].created, derived['head'].modified) == (
            original['head'].created, original['head'].modified)
        assert (actual.xMin, actual.yMin, actual.xMax, actual.yMax) == (80, 0, 790, 729)
        glyph_changes = changed_glyphs(original, derived)
        assert glyph_changes == ['M'] and len(original.getGlyphOrder()) - 1 == 2910
        changed_tables = sorted(tag for tag in original.reader.keys()
                                if original.getTableData(tag) != derived.getTableData(tag))
        assert set(changed_tables).issubset({'glyf', 'head', 'loca', 'name'})
        assert {'glyf', 'head', 'name'}.issubset(changed_tables)
        head_a, head_b = original.getTableData('head'), derived.getTableData('head')
        assert [i for i, (a, b) in enumerate(zip(head_a, head_b)) if a != b] == [8, 9, 10, 11]
        for tag in ('hmtx', 'maxp', 'prep', 'fpgm', 'cvt '):
            assert original.getTableData(tag) == derived.getTableData(tag), tag
        assert derived['maxp'].maxPoints >= 15 and derived['maxp'].maxSizeOfInstructions >= 138
        for a, b in zip(original['name'].names, derived['name'].names):
            assert (a.nameID, a.platformID, a.platEncID, a.langID) == (
                b.nameID, b.platformID, b.platEncID, b.langID)
            if a.nameID not in (1, 3, 4, 6):
                assert a.string == b.string, f'name record {a.nameID} changed'
        assert sha(Path(temp).read_bytes()) == OUTPUT_SHA256, 'derived font hash changed'
        assert changed_tables == audit_record()['changed_tables']
        os.chmod(temp, 0o644)
        os.replace(temp, output)
        atomic_audit(audit)
    finally:
        Path(temp).unlink(missing_ok=True)


def main():
    p = argparse.ArgumentParser()
    p.add_argument('source', type=Path)
    p.add_argument('output', type=Path)
    p.add_argument('audit', type=Path)
    args = p.parse_args()
    build(args.source, args.output, args.audit)


if __name__ == '__main__':
    main()
